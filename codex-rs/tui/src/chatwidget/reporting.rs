use codex_protocol::ThreadId;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Default)]
pub(crate) struct PersistedScope {
    #[serde(default)]
    mode: String,
    #[serde(default)]
    allowed_hosts: Vec<String>,
    #[serde(default)]
    allowed_domains: Vec<String>,
    #[serde(default)]
    notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct PersistedEvidence {
    #[serde(default)]
    name: String,
    #[serde(default)]
    path: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub(crate) struct PersistedFinding {
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) target: String,
    #[serde(default)]
    pub(crate) vulnerability: String,
    #[serde(default)]
    pub(crate) severity: String,
    #[serde(default)]
    pub(crate) confidence: String,
    #[serde(default)]
    pub(crate) status: String,
    #[serde(default)]
    pub(crate) evidence: Vec<String>,
    #[serde(default)]
    pub(crate) reproduction: Option<String>,
    #[serde(default)]
    pub(crate) impact: Option<String>,
    #[serde(default)]
    pub(crate) limitations: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct PersistedSecurityState {
    #[serde(default)]
    scope: PersistedScope,
    #[serde(default)]
    evidence_index: Vec<PersistedEvidence>,
    #[serde(default)]
    findings: Vec<PersistedFinding>,
}

pub(crate) enum ReportTarget {
    All,
    Finding(String),
}

pub(crate) fn parse_report_target(args: &str) -> Result<ReportTarget, String> {
    let trimmed = args.trim();
    if trimmed.is_empty() || trimmed == "all" {
        return Ok(ReportTarget::All);
    }

    let parts = trimmed.split_whitespace().collect::<Vec<_>>();
    if parts.len() == 2 && parts[0] == "finding" {
        return Ok(ReportTarget::Finding(parts[1].to_string()));
    }

    Err("Usage: /report [all|finding <id>]".to_string())
}

pub(crate) fn findings_summary(codex_home: &Path, thread_id: ThreadId) -> Result<String, String> {
    let state = load_security_state(codex_home, thread_id)?;
    if state.findings.is_empty() {
        return Ok("No findings recorded for this session.".to_string());
    }

    let mut lines = vec![
        format!("Current findings ({}):", state.findings.len()),
        String::new(),
    ];
    for (index, finding) in state.findings.iter().enumerate() {
        let id = finding_id(finding, index);
        lines.push(format!(
            "- {}: {} on {} [{} / {} / {}]",
            id,
            finding.vulnerability,
            finding.target,
            finding.severity,
            finding.confidence,
            finding.status
        ));
    }
    Ok(lines.join("\n"))
}

pub(crate) fn write_markdown_report(
    codex_home: &Path,
    thread_id: ThreadId,
    target: ReportTarget,
) -> Result<PathBuf, String> {
    let mut state = load_security_state(codex_home, thread_id)?;
    for (index, finding) in state.findings.iter_mut().enumerate() {
        if finding.id.trim().is_empty() {
            finding.id = format!("finding-{:04}", index + 1);
        }
    }

    match target {
        ReportTarget::All => {
            let report = render_markdown_report(&state);
            let path = session_root_dir(codex_home, thread_id).join("report.md");
            fs::write(&path, report).map_err(|err| format!("failed to write report: {err}"))?;
            Ok(path)
        }
        ReportTarget::Finding(id) => {
            let Some(finding) = state
                .findings
                .iter()
                .find(|finding| finding.id == id)
                .cloned()
            else {
                return Err(format!("Finding `{id}` was not found."));
            };
            state.findings = vec![finding];
            let report = render_markdown_report(&state);
            let path =
                session_root_dir(codex_home, thread_id).join(format!("report-finding-{}.md", id));
            fs::write(&path, report)
                .map_err(|err| format!("failed to write finding report: {err}"))?;
            Ok(path)
        }
    }
}

fn load_security_state(
    codex_home: &Path,
    thread_id: ThreadId,
) -> Result<PersistedSecurityState, String> {
    let root = session_root_dir(codex_home, thread_id);
    let state_path = root.join("state.json");
    let findings_path = root.join("findings.json");

    let mut state = read_json::<PersistedSecurityState>(&state_path).unwrap_or_default();
    let findings = read_json::<Vec<PersistedFinding>>(&findings_path);
    if let Some(findings) = findings {
        state.findings = findings;
    }
    Ok(state)
}

fn read_json<T: for<'de> serde::Deserialize<'de>>(path: &Path) -> Option<T> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn session_root_dir(codex_home: &Path, thread_id: ThreadId) -> PathBuf {
    codex_home.join("security").join(thread_id.to_string())
}

fn finding_id(finding: &PersistedFinding, index: usize) -> String {
    if finding.id.trim().is_empty() {
        format!("finding-{:04}", index + 1)
    } else {
        finding.id.clone()
    }
}

fn render_markdown_report(state: &PersistedSecurityState) -> String {
    let mut lines = vec!["# Security Assessment Report".to_string(), String::new()];
    lines.push("## Scope".to_string());
    lines.push(format!(
        "- Mode: {}",
        if state.scope.mode.is_empty() {
            "host_only"
        } else {
            state.scope.mode.as_str()
        }
    ));
    if !state.scope.allowed_hosts.is_empty() {
        lines.push(format!(
            "- Allowed hosts: {}",
            state.scope.allowed_hosts.join(", ")
        ));
    }
    if !state.scope.allowed_domains.is_empty() {
        lines.push(format!(
            "- Allowed domains: {}",
            state.scope.allowed_domains.join(", ")
        ));
    }
    if let Some(notes) = state.scope.notes.as_deref() {
        lines.push(format!("- Notes: {notes}"));
    }
    lines.push(String::new());
    lines.push("## Findings".to_string());
    if state.findings.is_empty() {
        lines.push("- No confirmed findings recorded.".to_string());
    } else {
        for (index, finding) in state.findings.iter().enumerate() {
            let id = finding_id(finding, index);
            lines.push(format!(
                "- {} on {} [{} / {} / {}] ({id})",
                finding.vulnerability,
                finding.target,
                finding.severity,
                finding.confidence,
                finding.status
            ));
            if let Some(impact) = finding.impact.as_deref() {
                lines.push(format!("  Impact: {impact}"));
            }
            if let Some(reproduction) = finding.reproduction.as_deref() {
                lines.push(format!("  Reproduction: {reproduction}"));
            }
            if let Some(limitations) = finding.limitations.as_deref() {
                lines.push(format!("  Limitations: {limitations}"));
            }
            if !finding.evidence.is_empty() {
                lines.push(format!("  Evidence: {}", finding.evidence.join(", ")));
            }
        }
    }
    lines.push(String::new());
    lines.push("## Evidence".to_string());
    if state.evidence_index.is_empty() {
        lines.push("- No evidence artifacts captured.".to_string());
    } else {
        for evidence in &state.evidence_index {
            lines.push(format!("- {}: {}", evidence.name, evidence.path));
        }
    }
    lines.join("\n")
}
