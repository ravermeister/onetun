# Copilot decisions and preferences

Record only concise, reusable user preferences and confirmed project decisions.
Do not record full chat transcripts, secrets, or sensitive personal content.

## User preferences

- Apply the repository's Copilot instructions and relevant specialist guidance
  to each task.
- Preserve durable, reusable decisions for future tasks and update related
  Copilot instructions, agents, or skills when those decisions change.
- Do not store full conversation transcripts; store only transferable decisions
  and preferences.

## Confirmed project guidance

- For Rust IDE diagnostics, verify the relevant Cargo target and feature
  configuration with Cargo before changing code based only on an IDE warning.
- Develop and test features on branches and pull requests; open in-progress
  pull requests as drafts. Every pull request merged into `main` triggers a
  release with concise GitHub-generated notes on both the PR and GitHub release.
- Determine release SemVer from the Conventional Commit PR title: `feat:` is
  minor, `!` or `BREAKING CHANGE:` is major, and other changes are patch.
  Store the latest released version in the repository Actions variable
  `RELEASE_VERSION` without a `v` prefix.
