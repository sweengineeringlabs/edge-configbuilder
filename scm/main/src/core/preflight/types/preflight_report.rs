use std::fmt;

use crate::{PreflightIssueKind, PreflightReport};

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
