# Copilot instructions

## Project

`tunone` (repository name `onetun`) is a cross-platform, user-space WireGuard
port forwarder. It uses `boringtun` for WireGuard and `smoltcp` as a virtual
TCP/IP stack to forward local TCP and experimental UDP traffic to WireGuard
peers, without requiring local root access or configuring system interfaces.
The Rust package uses edition 2024 and Tokio; stable Rust is used by CI.

## Persistent preferences and learning

- At the start of every task, apply these repository instructions and read
  `.copilot/DECISIONS.md` for durable user preferences and project decisions.
  Relevant custom agents and skills are available under `.github/agents/` and
  `.github/skills/`; use their guidance when working in those areas.
- Treat explicit user corrections and decisions as authoritative for the
  current task. If a preference or decision changes, use the latest explicit
  direction and update the decision note when it is reusable in future tasks.
- Learn only from clear, reusable decisions, confirmed corrections, or
  established project conventions. Do not infer a preference from a one-off
  request or record speculative conclusions.
- When a durable decision changes, update `.copilot/DECISIONS.md` and any
  directly affected instructions, agents, or skills so they remain consistent.
  Keep the note concise and remove superseded guidance.
- Do not save full chat transcripts, private or sensitive user content, or
  secrets. Copilot customizations do not provide automatic access to or
  recording of conversations outside the current available context.
- Do not edit customization files for every prompt. Make a persistent update
  only when a clear, reusable preference, decision, or correction warrants it.

## Architecture map

- `src/config.rs`: CLI and tunnel configuration parsing and validation.
- `src/main.rs`: binary startup, logging, and runtime entry point.
- `src/lib.rs`: starts the WireGuard and virtual-interface tasks.
- `src/wg.rs`, `src/events.rs`: WireGuard transport and event bus.
- `src/tunnel/`: local TCP/UDP listener and forwarding tasks.
- `src/virtual_iface/`: `smoltcp` TCP/UDP virtual interfaces and packet flow.
- `src/virtual_device.rs`: packet device connecting the virtual stack and event bus.
- `tests/wireguard_roundtrip.rs`: localhost WireGuard/TCP integration coverage.
- `.github/workflows/build.yml`: CI checks, tests, formatting, and Clippy.
- `.github/workflows/release.yml`: tagged release, crate, binary, and Docker publishing.

Keep protocol-specific changes in their existing TCP or UDP paths unless there
is a clear reason to share code. UDP is experimental. Preserve IPv4/IPv6 and
cross-platform behavior.

## Build and validation

Use the locked dependency graph and run the narrowest applicable checks. CI
currently runs:

```sh
cargo fmt --all -- --check
cargo check --locked
cargo check --no-default-features --locked
cargo test --locked
cargo test --no-default-features --locked
cargo clippy --locked -- -D warnings
```

Run formatting, both checks, both test commands, and Clippy for broad Rust
changes. Tests should use local/ephemeral resources and must not require access
to a real VPN peer, privileged network configuration, or production secrets.

## Engineering constraints

- Follow existing Rust, Tokio, `smoltcp`, and error-context patterns.
- Keep asynchronous task failures observable; do not silently discard errors.
- Treat packet lengths, queue/buffer limits, timeouts, address families, and
  socket lifetimes as correctness requirements, not implementation details.
- Do not implement cryptographic primitives or weaken WireGuard validation.
  Keep key material out of logs, fixtures, and committed files.
- Add or update tests for behavior changes, including error paths and relevant
  IPv4/IPv6 or TCP/UDP cases.
- Do not publish crates, push tags, create releases, or push Docker images
  unless the user explicitly requests that external action. Before any release,
  inspect the current workflow and verify the exact version and target.
