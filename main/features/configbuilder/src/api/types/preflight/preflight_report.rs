use std::fmt;

use crate::api::types::preflight::preflight_issue::PreflightIssue;
use crate::api::types::preflight::preflight_issue_kind::PreflightIssueKind;

/// Aggregated result of a [`preflight!`] dry-run.
///
/// Collects every issue found across all sections without stopping at the
/// first failure, so operators can resolve all problems in one pass.
///
/// # Example
///
/// ```rust,ignore
/// let report = preflight!(&loader, CacheConfig, BrokerConfig);
/// if !report.is_ok() {
///     eprintln!("{}", report);
/// }
/// ```
///
/// [`preflight!`]: crate::preflight
pub struct PreflightReport {
    issues: Vec<PreflightIssue>,
}

impl PreflightReport {
    /// Create an empty report with no issues.
    pub fn new() -> Self {
        Self { issues: Vec::new() }
    }

    /// Append an issue to the report.
    pub fn push(&mut self, issue: PreflightIssue) {
        self.issues.push(issue);
    }

    /// Whether the preflight found no issues.
    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    /// All issues found during the dry-run.
    pub fn issues(&self) -> &[PreflightIssue] {
        &self.issues
    }

    /// Total number of issues.
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }
}

impl Default for PreflightReport {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PreflightReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.issues.is_empty() {
            return write!(f, "preflight: OK");
        }
        writeln!(f, "preflight: {} issue(s)", self.issues.len())?;
        for issue in &self.issues {
            let tag = match issue.kind {
                PreflightIssueKind::LoadError => "LOAD",
                PreflightIssueKind::ValidationError => "VALIDATION",
                PreflightIssueKind::DependencyMissing => "DEPENDENCY",
                PreflightIssueKind::DependencyCycle => "CYCLE",
            };
            writeln!(f, "  [{tag}] {}: {}", issue.section, issue.message)?;
        }
        Ok(())
    }
}
