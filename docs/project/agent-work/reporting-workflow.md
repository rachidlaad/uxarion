# Reporting workflow implementation note

- `/findings` and `/report` now route through deterministic prompt scaffolds in the TUI, while `report_write` gained optional `finding_id` support so the existing security session state service remains the single writer for both `report.md` and per-finding Markdown artifacts under the same session directory.
