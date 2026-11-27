---
name: low-level-networking
description: Helps reason about tunone's packet device, smoltcp interfaces, buffer limits, resource lifetimes, and cross-platform user-space networking. Use for packet processing, memory, or systems-boundary changes.
---

## Systems review

Trace data and ownership across local sockets, virtual TCP/IP sockets, the
`VirtualIpDevice`, the event bus, and WireGuard. Inspect the existing limits and
address-count configuration in `Cargo.toml`; do not change capacities without
understanding memory and protocol consequences.

For each change, reason about:

- Bounds and validation for untrusted packet lengths and addresses.
- Bounded allocation, queue pressure, ordering, and backpressure.
- Socket and task lifetime, cancellation, shutdown, and cleanup.
- TCP stream versus UDP datagram behavior.
- IPv4/IPv6 conversion and platform-independent user-space operation.
- Whether a safe API can replace any proposed `unsafe` code.

Add a regression test for the discovered boundary or lifecycle bug. Prefer
local test resources and avoid root access or changes to host network state.
Measure performance claims rather than relying on intuition.
