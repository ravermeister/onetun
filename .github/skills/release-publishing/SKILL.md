---
name: release-publishing
description: Prepares and validates tunone crate, binary, GitHub, and Docker releases by checking versions, tags, workflows, and package contents. Use for release readiness or publishing requests.
---

## Release readiness

1. Read the current `.github/workflows/release.yml`; it defines the actual
   release targets and external effects.
2. Confirm the requested version, `Cargo.toml`/`Cargo.lock` consistency, tag
   format, package files, release notes, and target matrix.
3. Run relevant non-publishing checks first, including locked tests and
   `cargo package --locked --list` or `cargo package --locked` where appropriate.
   Verify command options against the installed Cargo version.
4. Report exactly what publishing steps will affect: crates.io, GitHub release
   assets, and/or Docker Hub. Never claim a remote action succeeded based only
   on local validation.

Do not push tags, call `cargo publish`, create a GitHub release, or push
containers unless the user explicitly authorizes that exact external action.
Keep tokens and secrets out of output and repository files.
