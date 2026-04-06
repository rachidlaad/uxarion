# Reporting Workflow Implementation Note

- Added stable `finding-*` IDs in persisted security findings (including backfill for legacy findings without IDs), plus TUI `/findings` and `/report` slash-command flows that read existing session state and trigger Markdown report generation (`/report`, `/report all`, `/report finding <id>`) through the existing `report_write` tool path.
