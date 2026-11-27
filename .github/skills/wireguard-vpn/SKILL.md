---
name: wireguard-vpn
description: Provides safe implementation guidance for tunone's WireGuard peer configuration and boringtun packet transport. Use for VPN, handshake, key, endpoint, keepalive, or encrypted-packet changes.
---

## Protocol boundary

`boringtun` owns WireGuard cryptography. Keep changes in the integration layer;
do not implement primitives, modify cryptographic algorithms, or weaken peer
validation. Trace configuration in `src/config.rs`, transport in `src/wg.rs`,
event delivery in `src/events.rs`, and packet handling in `src/virtual_device.rs`
and `src/virtual_iface/`.

## Safe change procedure

1. Identify the exact state transition or packet path affected.
2. Preserve key parsing, peer identity, handshake/retry/keepalive behavior, and
   the distinction between encrypted network packets and decrypted IP packets.
3. Never log or commit private keys, preshared keys, or production endpoint
   configuration. Use deterministic test-only keys and localhost ephemeral
   sockets.
4. Test failure cases as well as successful packet flow. Extend
   `tests/wireguard_roundtrip.rs` when it is a suitable integration boundary.
5. State interoperability/security assumptions and run applicable locked Cargo
   checks and tests.

Do not infer that a successful unit test proves peer interoperability or
cryptographic correctness.
