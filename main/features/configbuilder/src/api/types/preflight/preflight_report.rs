use std::fmt;

use crate::api::error::config_error::ConfigError;

/// Category of a preflight issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreflightIssueKind {
    /// The section could not be loaded (I/O or parse error).
    LoadError,
    /// The section loaded but failed cross-field validation.
    ValidationError,
    /// A declared dependency is absent or disabled.
    DependencyMissing,
    /// A cycle exists in the declared dependency graph.
    DependencyCycle,
}

impl PreflightIssueKind {
    /// Classify a [`ConfigError`] into the appropriate issue kind.
    ///
    /// Used by the [`preflight!`] macro to convert load errors into report entries.
    ///
    /// [`preflight!`]: crate::preflight
    pub fn from_config_error(e: &ConfigError) -> Self {
        match e {
            ConfigError::Parse(_) | ConfigError::Io(_) | ConfigError::NotFound(_) => {
                Self::LoadError
            }
            ConfigError::Validation { .. } => Self::ValidationError,
        }
    }
}

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
