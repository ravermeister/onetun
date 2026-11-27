---
name: testing-validation
description: Selects and runs reliable Rust validation for tunone, including default and no-default Cargo features, integration tests, rustfmt, and Clippy. Use when implementing tests or validating changes.
---

## Validation commands

The build workflow (`.github/workflows/build.yml`) currently runs:

```sh
cargo fmt --all -- --check
cargo check --locked
cargo check --no-default-features --locked
cargo test --locked
cargo test --no-default-features --locked
cargo clippy --locked -- -D warnings
```

Start with the command that most directly covers the changed module, then run
all relevant CI checks. Report failures as failures; do not replace them with a
success-shaped fallback.

## Test design

- Follow local unit-test and `tests/wireguard_roundtrip.rs` patterns.
- Use deterministic inputs, local ephemeral sockets, and bounded timeouts.
- Test observable behavior, error paths, cleanup, and feature gates.
- For networking, consider protocol, address family, empty and large payloads,
  timeout, packet/datagram boundaries, and task termination.
- Keep tests independent of real VPN peers, secrets, root access, and host
  network configuration.
- Explain the limits of mocks and the behavior not covered by the test.
