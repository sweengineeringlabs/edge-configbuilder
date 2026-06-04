use crate::api::error::config_error::ConfigError;

/// Category of a preflight issue.
///
/// Used to tag each [`PreflightIssue`] so operators can quickly triage whether
/// a startup problem is a file-system error, a config value error, or a
/// feature-dependency ordering problem.
///
/// [`PreflightIssue`]: crate::PreflightIssue
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::{ConfigError, PreflightIssueKind};
///
/// // Classify a parse error as a LoadError.
/// let err = ConfigError::Parse("unexpected token".to_string());
/// assert_eq!(
///     PreflightIssueKind::from_config_error(&err),
///     PreflightIssueKind::LoadError,
/// );
///
/// // Classify a validation error.
/// let err = ConfigError::validation("auth", "cert_path required");
/// assert_eq!(
///     PreflightIssueKind::from_config_error(&err),
///     PreflightIssueKind::ValidationError,
/// );
/// ```
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
