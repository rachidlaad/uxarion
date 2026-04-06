use codex_protocol::ThreadId;
use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ReportScope<'a> {
    All,
    Finding(&'a str),
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct PersistedFinding {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub vulnerability: String,
    #[serde(default)]
    pub severity: String,
    #[serde(default)]
    pub confidence: String,
    #[serde(default)]
    pub status: String,
}

pub(super) fn parse_report_scope(args: &str) -> Result<ReportScope<'_>, String> {
    let trimmed = args.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("all") {
        return Ok(ReportScope::All);
    }

    let mut parts = trimmed.split_whitespace();
    let command = parts.next();
    let maybe_id = parts.next();
    let extra = parts.next();

    if matches!(command, Some(value) if value.eq_ignore_ascii_case("finding"))
        && let Some(id) = maybe_id
        && extra.is_none()
    {
        return Ok(ReportScope::Finding(id));
    }

    Err("Usage: /report [all|finding <id>]".to_string())
}

pub(super) fn security_session_dir(
    codex_home: &Path,
    thread_id: Option<ThreadId>,
) -> Option<PathBuf> {
    thread_id.map(|id| codex_home.join("security").join(id.to_string()))
}

pub(super) fn load_findings(
    codex_home: &Path,
    thread_id: Option<ThreadId>,
) -> Result<Vec<PersistedFinding>, String> {
    let Some(session_dir) = security_session_dir(codex_home, thread_id) else {
        return Ok(Vec::new());
    };

    let findings_path = session_dir.join("findings.json");
    if !findings_path.exists() {
        return Ok(Vec::new());
    }

    let bytes = std::fs::read(&findings_path).map_err(|err| {
        format!(
            "Failed to read findings from {}: {err}",
            findings_path.display()
        )
    })?;
    let mut findings: Vec<PersistedFinding> = serde_json::from_slice(&bytes).map_err(|err| {
        format!(
            "Failed to parse findings from {}: {err}",
            findings_path.display()
        )
    })?;

    for index in 0..findings.len() {
        if findings[index].id.trim().is_empty() {
            findings[index].id = format!("finding-{:04}", index + 1);
        }
    }

    Ok(findings)
}

pub(super) fn format_findings_summary(findings: &[PersistedFinding]) -> String {
    if findings.is_empty() {
        return "No findings have been recorded yet.".to_string();
    }

    let mut lines = vec![format!("{} finding(s):", findings.len())];
    for finding in findings {
        lines.push(format!(
            "- [{}] {} on {} [{} / {} / {}]",
            finding.id,
            finding.vulnerability,
            finding.target,
            finding.severity,
            finding.confidence,
            finding.status
        ));
    }
    lines.join("\n")
}

pub(super) fn report_prompt(scope: ReportScope<'_>) -> String {
    match scope {
        ReportScope::All => "Write a Markdown security report now by calling `report_write` with `include_evidence=true` and no `finding_id`. After writing the file, tell me the exact `report_path`.".to_string(),
        ReportScope::Finding(id) => format!(
            "Write a Markdown report for finding `{id}` now by calling `report_write` with `include_evidence=true` and `finding_id` set to `{id}`. After writing the file, tell me the exact `report_path`."
        ),
    }
}
