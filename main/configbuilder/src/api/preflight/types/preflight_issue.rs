use crate::api::preflight::types::preflight_issue_kind::PreflightIssueKind;

/// A single issue captured during a [`preflight!`] dry-run.
///
/// Collected into a [`PreflightReport`] rather than returned as an error, so
/// the preflight can enumerate every issue in one pass instead of stopping at
/// the first failure.
///
/// [`preflight!`]: crate::preflight
/// [`PreflightReport`]: crate::PreflightReport
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::{PreflightIssue, PreflightIssueKind};
///
/// let issue = PreflightIssue {
///     section: "message_broker".to_string(),
///     kind: PreflightIssueKind::ValidationError,
///     message: "cert_path is required when tls_enabled = true".to_string(),
/// };
///
/// assert_eq!(issue.section, "message_broker");
/// assert_eq!(issue.kind, PreflightIssueKind::ValidationError);
/// ```
#[derive(Debug, Clone)]
pub struct PreflightIssue {
    /// Config section that triggered the issue (e.g. `"message_broker"`).
    pub section: String,
    /// Category of the issue — determines how it is displayed in the report.
    pub kind: PreflightIssueKind,
    /// Human-readable description of what went wrong; must be actionable.
    pub message: String,
}
