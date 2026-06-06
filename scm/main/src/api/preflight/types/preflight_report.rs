use std::fmt;

use crate::api::preflight::types::preflight_issue::PreflightIssue;
use crate::api::preflight::types::preflight_issue_kind::PreflightIssueKind;

/// Aggregated result of a [`preflight!`] dry-run.
///
/// Collects every issue found across all sections without stopping at the
/// first failure, so operators can resolve all problems in one pass.
/// Implements [`Display`] so you can print it directly with `eprintln!("{}", report)`.
///
/// [`preflight!`]: crate::preflight
/// [`Display`]: std::fmt::Display
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::{PreflightIssue, PreflightIssueKind, PreflightReport};
///
/// let mut report = PreflightReport::new();
/// assert!(report.is_ok());
/// assert_eq!(report.issue_count(), 0);
///
/// report.push(PreflightIssue {
///     section: "cache".to_string(),
///     kind: PreflightIssueKind::ValidationError,
///     message: "max_size must be > 0".to_string(),
/// });
///
/// assert!(!report.is_ok());
/// assert_eq!(report.issue_count(), 1);
/// assert_eq!(report.issues()[0].section, "cache");
/// ```
pub struct PreflightReport {
    issues: Vec<PreflightIssue>,
}

impl PreflightReport {
    /// Create an empty report with no issues.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::PreflightReport;
    /// let report = PreflightReport::new();
    /// assert!(report.is_ok());
    /// ```
    pub fn new() -> Self {
        Self { issues: Vec::new() }
    }

    /// Append an issue to the report.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::{PreflightIssue, PreflightIssueKind, PreflightReport};
    ///
    /// let mut report = PreflightReport::new();
    /// report.push(PreflightIssue {
    ///     section: "tls".to_string(),
    ///     kind: PreflightIssueKind::LoadError,
    ///     message: "cert file not found".to_string(),
    /// });
    /// assert_eq!(report.issue_count(), 1);
    /// ```
    pub fn push(&mut self, issue: PreflightIssue) {
        self.issues.push(issue);
    }

    /// Returns `true` when no issues were collected.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::PreflightReport;
    /// assert!(PreflightReport::new().is_ok());
    /// ```
    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    /// All issues found during the dry-run, in collection order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::{PreflightIssue, PreflightIssueKind, PreflightReport};
    ///
    /// let mut report = PreflightReport::new();
    /// report.push(PreflightIssue {
    ///     section: "auth".to_string(),
    ///     kind: PreflightIssueKind::DependencyMissing,
    ///     message: "requires 'tls'".to_string(),
    /// });
    /// assert_eq!(report.issues()[0].section, "auth");
    /// ```
    pub fn issues(&self) -> &[PreflightIssue] {
        &self.issues
    }

    /// Total number of issues collected.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::PreflightReport;
    /// let report = PreflightReport::new();
    /// assert_eq!(report.issue_count(), 0);
    /// ```
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
