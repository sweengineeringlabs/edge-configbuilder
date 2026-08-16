/// Category of a preflight issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreflightIssueKind {
    /// I/O, parsing, or unreadable file failure during load.
    LoadError,
    /// Section loaded successfully but failed validation.
    ValidationError,
    /// A dependency declared by a feature was not enabled.
    DependencyMissing,
    /// The dependency graph contains a cycle.
    DependencyCycle,
}
