---
name: rust-systems
description: Guides Rust implementation in tunone, including Tokio async tasks, ownership, error propagation, feature gates, and efficient bounded I/O. Use when changing or reviewing Rust code.
---

## Workflow

1. Trace the public entry point and the relevant module call path before editing.
   The main architecture is summarized in `.github/copilot-instructions.md`.
2. Follow the repository's Rust edition, dependency, Tokio, and `anyhow`
   conventions. Prefer safe Rust, explicit ownership, and bounded resources.
3. Keep feature-gated builds compiling, especially the library-only
   `--no-default-features` configuration.
4. Add focused tests for observable changes and error paths.
5. Run formatting and the narrowest locked Cargo checks/tests; run all CI
   commands for broad changes.

## Review checklist

- Are spawned-task errors observable, and is cancellation behavior intentional?
- Are allocations, queues, and buffer sizes bounded?
- Are public APIs and feature combinations preserved?
- Are errors contextualized instead of swallowed or disguised?
- Are new dependencies necessary and compatible with the project's build targets?
