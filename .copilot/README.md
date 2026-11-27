# Copilot customizations

GitHub Copilot discovers repository customizations from `.github`, not from a
root-level `.copilot` directory. This directory is an index; the active
repository configuration lives in:

- [Repository instructions](../.github/copilot-instructions.md)
- [Custom agents](../.github/agents/)
- [Agent skills](../.github/skills/)

The skills are tailored to tunone and follow the Agent Skills format. The
curated directory at [ai-skills.io](https://ai-skills.io/) informed the
specialist-skill approach; no third-party skill content or scripts were copied.

Durable preferences and project decisions are recorded in
[DECISIONS.md](DECISIONS.md). Copilot should consult it for each task and
update it only when a clear, reusable decision changes; full chat transcripts
are not stored.

References:

- [GitHub Copilot customization cheat sheet](https://docs.github.com/en/copilot/reference/customization-cheat-sheet)
- [Agent Skills specification](https://agentskills.io/specification)
