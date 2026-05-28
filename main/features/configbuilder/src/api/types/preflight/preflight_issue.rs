use crate::api::types::preflight::preflight_issue_kind::PreflightIssueKind;

/// A single issue captured during a [`preflight!`] dry-run.
///
/// [`preflight!`]: crate::preflight
#[derive(Debug, Clone)]
pub struct PreflightIssue {
    /// Config section that triggered the issue.
    pub section: String,
    /// Category of the issue.
    pub kind: PreflightIssueKind,
    /// Human-readable description of what went wrong.
    pub message: String,
}
