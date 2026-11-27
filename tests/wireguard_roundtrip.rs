use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::time::Duration;

use boringtun::noise::{Tunn, TunnResult};
use smoltcp::phy::ChecksumCapabilities;
use smoltcp::wire::{
    IpAddress, IpProtocol, Ipv4Address, Ipv4Packet, Ipv4Repr, TcpControl, TcpPacket, TcpRepr,
    TcpSeqNumber,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tunone::config::{Config, PortForwardConfig, PortProtocol, PublicKey, StaticSecret};
use tunone::events::Bus;
use tunone::start_tunnels;

const CLIENT_IP: Ipv4Addr = Ipv4Addr::new(10, 85, 150, 4);
const SERVER_IP: Ipv4Addr = Ipv4Addr::new(10, 85, 4, 13);
const SERVER_PORT: u16 = 7080;
const RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK";

#[tokio::test]
async fn tcp_forward_completes_wireguard_handshake_and_receives_peer_response() {
    let client_private = StaticSecret::from([7; 32]);
    let client_public = PublicKey::from(&client_private);
    let server_private = StaticSecret::from([9; 32]);
    let server_public = PublicKey::from(&server_private);

    let peer_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let endpoint_addr = peer_socket.local_addr().unwrap();
    let peer = Tunn::new(server_private, client_public, None, None, 0, None);
    let peer_task = tokio::spawn(run_peer(peer_socket, peer));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let local_port = listener.local_addr().unwrap().port();
    drop(listener);

    let config = Config {
        port_forwards: vec![PortForwardConfig {
            source: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, local_port)),
            destination: SocketAddr::V4(SocketAddrV4::new(SERVER_IP, SERVER_PORT)),
            protocol: PortProtocol::Tcp,
            remote: false,
        }],
        remote_port_forwards: vec![],
        private_key: Arc::new(client_private),
        endpoint_public_key: Arc::new(server_public),
        preshared_key: None,
        endpoint_addr,
        endpoint_bind_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)),
        source_peer_ip: CLIENT_IP.into(),
        keepalive_seconds: None,
        max_transmission_unit: 1420,
        log: "trace".into(),
        warnings: vec![],
        pcap_file: None,
    };

    start_tunnels(config, Bus::default()).await.unwrap();

    let mut client = tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            match TcpStream::connect(("127.0.0.1", local_port)).await {
                Ok(stream) => break stream,
                Err(_) => tokio::time::sleep(Duration::from_millis(10)).await,
            }
        }
    })
    .await
    .expect("tunone did not start its local TCP listener");

    client
        .write_all(b"GET / HTTP/1.1\r\nHost: test\r\n\r\n")
        .await
        .unwrap();

    let mut response = vec![0; RESPONSE.len()];
    tokio::time::timeout(Duration::from_secs(5), client.read_exact(&mut response))
        .await
        .expect("timed out waiting for the WireGuard peer response")
        .expect("failed to read the response");
    assert_eq!(response, RESPONSE);

    peer_task.abort();
}

async fn run_peer(socket: UdpSocket, mut peer: Tunn) {
    let mut recv_buf = [0; 65536];
    let mut wg_buf = [0; 65536];

    loop {
        let (size, remote_addr) = socket.recv_from(&mut recv_buf).await.unwrap();
        let mut datagram = &recv_buf[..size];

        loop {
            let result = peer.decapsulate(None, datagram, &mut wg_buf);
            datagram = &[];
            match result {
                TunnResult::WriteToNetwork(packet) => {
                    let packet = packet.to_vec();
                    socket.send_to(&packet, remote_addr).await.unwrap();
                }
                TunnResult::WriteToTunnelV4(packet, _) => {
                    if let Some(response) = tcp_response(packet) {
                        let result = peer.encapsulate(&response, &mut wg_buf);
                        if let TunnResult::WriteToNetwork(packet) = result {
                            let packet = packet.to_vec();
                            socket.send_to(&packet, remote_addr).await.unwrap();
                        }
                    }
                }
                TunnResult::Done | TunnResult::Err(_) => break,
                TunnResult::WriteToTunnelV6(_, _) => {}
            }
        }
    }
}

fn tcp_response(packet: &[u8]) -> Option<Vec<u8>> {
    let ip_packet = Ipv4Packet::new_checked(packet).ok()?;
    let source_ip = ip_packet.src_addr();
    let destination_ip = ip_packet.dst_addr();
    let tcp_packet = TcpPacket::new_checked(ip_packet.payload()).ok()?;
    let incoming = TcpRepr::parse(
        &tcp_packet,
        &IpAddress::Ipv4(source_ip),
        &IpAddress::Ipv4(destination_ip),
        &ChecksumCapabilities::default(),
    )
    .ok()?;

    let (control, seq_number, ack_number, payload) = if incoming.control == TcpControl::Syn {
        (
            TcpControl::Syn,
            TcpSeqNumber(1000),
            TcpSeqNumber(incoming.seq_number.0.wrapping_add(1)),
            &[][..],
        )
    } else if !incoming.payload.is_empty() {
        (
            TcpControl::Psh,
            TcpSeqNumber(1001),
            TcpSeqNumber(
                incoming
                    .seq_number
                    .0
                    .wrapping_add(incoming.payload.len() as i32),
            ),
            RESPONSE,
        )
    } else {
        return None;
    };

    let [source_a, source_b, source_c, source_d] = destination_ip.octets();
    let [destination_a, destination_b, destination_c, destination_d] = source_ip.octets();
    let ip_source = Ipv4Address::new(source_a, source_b, source_c, source_d);
    let ip_destination =
        Ipv4Address::new(destination_a, destination_b, destination_c, destination_d);
    let source = IpAddress::Ipv4(ip_source);
    let destination = IpAddress::Ipv4(ip_destination);
    let tcp_repr = TcpRepr {
        src_port: incoming.dst_port,
        dst_port: incoming.src_port,
        control,
        seq_number,
        ack_number: Some(ack_number),
        window_len: 65535,
        window_scale: None,
        max_seg_size: None,
        sack_permitted: false,
        sack_ranges: [None, None, None],
        payload,
        timestamp: None,
    };

    let mut response = vec![0; 20 + tcp_repr.buffer_len()];
    {
        let mut tcp_packet = TcpPacket::new_unchecked(&mut response[20..]);
        tcp_repr.emit(
            &mut tcp_packet,
            &source,
            &destination,
            &ChecksumCapabilities::default(),
        );
    }
    let ipv4_repr = Ipv4Repr {
        src_addr: ip_source,
        dst_addr: ip_destination,
        next_header: IpProtocol::Tcp,
        payload_len: tcp_repr.buffer_len(),
        hop_limit: 64,
    };
    ipv4_repr.emit(
        &mut Ipv4Packet::new_unchecked(&mut response),
        &ChecksumCapabilities::default(),
    );
    Some(response)
}
