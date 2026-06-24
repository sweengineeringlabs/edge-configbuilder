//! API contract for preflight report types produced by the `preflight!` macro.

use crate::api::{PreflightIssue, PreflightIssueKind, PreflightReport};

/// Contract binding the preflight report type family.
pub trait Preflight {
    /// A single preflight issue.
    type Issue: Into<PreflightIssue>;

    /// Machine-readable issue classification.
    type IssueKind: Into<PreflightIssueKind>;

    /// Aggregate report returned by preflight checks.
    type Report: Into<PreflightReport>;
}
