use crate::api::preflight::types::preflight_issue::PreflightIssue;

/// Aggregated result of a [`preflight!`] dry-run.
pub struct PreflightReport {
    pub(crate) issues: Vec<PreflightIssue>,
}
