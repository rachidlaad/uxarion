#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ReportTarget {
    SessionAll,
    SingleFinding(String),
}

pub(super) fn parse_report_target(args: &str) -> Result<ReportTarget, String> {
    let trimmed = args.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("all") {
        return Ok(ReportTarget::SessionAll);
    }

    let mut parts = trimmed.split_whitespace();
    let first = parts.next();
    let second = parts.next();
    let trailing = parts.next();
    if first == Some("finding")
        && trailing.is_none()
        && let Some(id) = second
    {
        if id.trim().is_empty() {
            return Err("Usage: /report [all|finding <id>]".to_string());
        }
        return Ok(ReportTarget::SingleFinding(id.to_string()));
    }

    Err("Usage: /report [all|finding <id>]".to_string())
}

pub(super) fn findings_prompt() -> String {
    "List every current security finding in this session as concise bullet points using this format: `<finding_id> — <vulnerability> on <target> [<severity>/<confidence>/<status>]`. Do not invent IDs; use recorded finding IDs only. If none exist, say there are no findings.".to_string()
}

pub(super) fn report_prompt(target: &ReportTarget) -> String {
    match target {
        ReportTarget::SessionAll => "Call `report_write` with `include_evidence: true` and no `finding_id`, then respond with the saved `report_path` only.".to_string(),
        ReportTarget::SingleFinding(id) => format!(
            "Call `report_write` with `include_evidence: true` and `finding_id: \"{id}\"`, then respond with the saved `report_path` only."
        ),
    }
}
