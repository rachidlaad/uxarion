# Project Context

This directory is the maintainer and agent context layer for Uxarion.

Use it to understand:

- what is currently shipped
- how the product is structured
- how releases are produced
- what is still broken or intentionally deferred
- what important product decisions have already been made

Start here:

- [current-state.md](./current-state.md)
- [architecture.md](./architecture.md)
- [release-process.md](./release-process.md)
- [handoff.md](./handoff.md)
- [known-issues.md](./known-issues.md)
- [roadmap-context.md](./roadmap-context.md)

Decision records live in [decisions/](./decisions/).

Rules for this folder:

- Keep it current and concise.
- Prefer updating these files over repeating the same context in issues or PR descriptions.
- Do not put secrets, private tokens, or sensitive operator-only notes in tracked files here.
- For local-only or private AI notes, use ignored local files such as `.codex/` instead of tracked repo files.
