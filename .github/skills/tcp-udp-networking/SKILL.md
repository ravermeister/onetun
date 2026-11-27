---
name: tcp-udp-networking
description: Guides TCP and UDP forwarding changes in tunone, including smoltcp virtual sockets, port mapping, packet semantics, timeouts, and IPv4/IPv6. Use for transport-layer bugs or features.
---

## Trace before changing

Follow traffic from `src/tunnel/tcp.rs` or `src/tunnel/udp.rs`, through
`src/virtual_device.rs`, `src/virtual_iface/tcp.rs` or
`src/virtual_iface/udp.rs`, into `src/wg.rs`, and back. Check `src/config.rs`
for protocol and address-family rules.

## Protocol invariants

- TCP is a byte stream with connection state, teardown, possible half-close,
  and flow control. Preserve the mapping between a local connection and its
  virtual socket.
- UDP is datagram-oriented. Preserve message boundaries and destination/source
  mapping; explicitly consider the lifetime and cleanup of per-client state.
- Keep per-protocol socket behavior separate where their semantics differ.
- Consider IPv4, IPv6, cross-family forwarding, maximum packet size, queue
  capacity, timeout, and error paths.
- UDP remains experimental; do not imply otherwise in code or documentation.

Prefer local, ephemeral-socket tests with bounded waits. Cover responses,
timeouts/errors, and cleanup where applicable; run the associated locked Cargo
tests and checks.
