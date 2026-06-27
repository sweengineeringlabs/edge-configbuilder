use std::fmt;

use crate::{PreflightIssue, PreflightIssueKind, PreflightReport};

impl PreflightReport {
    /// Create an empty preflight report.
    pub(crate) fn new() -> Self {
        Self { issues: Vec::new() }
    }

    /// Add a preflight issue to the report.
    pub(crate) fn push(&mut self, issue: PreflightIssue) {
        self.issues.push(issue);
    }

    /// Return true when the report contains no issues.
    pub(crate) fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    /// Borrow the collected preflight issues.
    pub(crate) fn issues(&self) -> &[PreflightIssue] {
        &self.issues
    }

    /// Return the number of collected issues.
    pub(crate) fn issue_count(&self) -> usize {
        self.issues.len()
    }
}

impl crate::api::PreflightReportOps for PreflightReport {
    fn is_ok(&self) -> bool {
        PreflightReport::is_ok(self)
    }

    fn issues(&self) -> &[crate::api::PreflightIssue] {
        PreflightReport::issues(self)
    }

    fn issue_count(&self) -> usize {
        PreflightReport::issue_count(self)
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
