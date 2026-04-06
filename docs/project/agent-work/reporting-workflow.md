# Reporting workflow implementation note

- Reporting commands now read persisted security session state under `CODEX_HOME/security/<thread_id>/` to power `/findings`, write session-wide Markdown to `report.md`, and write per-finding Markdown to `report-finding-<id>.md` without introducing a second persistence format.
