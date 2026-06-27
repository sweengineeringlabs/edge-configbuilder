use crate::api::PreflightIssue;

/// Read-only queries on a [`PreflightReport`].
///
/// Implemented by [`PreflightReport`] in the `core/` layer.
///
/// [`PreflightReport`]: crate::PreflightReport
pub trait PreflightReportOps {
    /// Return `true` when the report contains no issues.
    fn is_ok(&self) -> bool;

    /// Borrow the collected preflight issues.
    fn issues(&self) -> &[PreflightIssue];

    /// Return the number of collected issues.
    fn issue_count(&self) -> usize;
}
