---
name: release-publishing
description: Prepares and validates tunone crate, binary, GitHub, and Docker releases by checking versions, tags, workflows, and package contents. Use for release readiness or publishing requests.
---

## Release readiness

1. Read the current `.github/workflows/release.yml`; it defines the actual
   release trigger, version bump rules, release targets, and external effects.
2. Releases are triggered by merged pull requests to `fork/main` (the current
   release branch) or `main`. The PR title uses Conventional Commits: `feat:`
   means minor, `!` or `BREAKING CHANGE:` means major, and all other changes
   mean patch. `RELEASE_VERSION` is the latest released SemVer without a `v`
   prefix; an unset variable falls back to the latest release tag.
3. Confirm the resulting version, `Cargo.toml`/`Cargo.lock` consistency, tag
   format, package files, generated release notes, and target matrix.
4. Run relevant non-publishing checks first, including locked tests and
   `cargo package --locked --list` or `cargo package --locked` where appropriate.
   Verify command options against the installed Cargo version.
5. Report exactly what publishing steps will affect: crates.io, GitHub release
   assets, and/or Docker Hub. Never claim a remote action succeeded based only
   on local validation.

The automated crate publishing step updates the checked-out manifest to the
release version and lets Cargo update `Cargo.lock` before publishing. It
intentionally uses `cargo publish --locked --allow-dirty` for those generated
version changes; do not remove `--allow-dirty` unless the release flow first
commits the version update.

Do not push tags, call `cargo publish`, create a GitHub release, or push
containers unless the user explicitly authorizes that exact external action.
Keep tokens and secrets out of output and repository files.
