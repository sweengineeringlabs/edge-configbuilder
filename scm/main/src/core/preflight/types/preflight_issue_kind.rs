//! [`PreflightIssueKind`] — category classifier for preflight issues.

use crate::api::{ConfigError, PreflightIssueKind, PreflightIssueKindOps};

impl PreflightIssueKind {
    pub(crate) fn from_config_error(e: &ConfigError) -> Self {
        match e {
            ConfigError::Parse(_) | ConfigError::Io(_) | ConfigError::NotFound(_) => {
                Self::LoadError
            }
            ConfigError::Validation { .. } => Self::ValidationError,
        }
    }
}

impl PreflightIssueKindOps for PreflightIssueKind {
    fn variant_name(&self) -> &'static str {
        match self {
            Self::LoadError => "LoadError",
            Self::ValidationError => "ValidationError",
            Self::DependencyMissing => "DependencyMissing",
            Self::DependencyCycle => "DependencyCycle",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::ConfigError;

    #[test]
    fn test_from_config_error_parse_error_returns_load_error() {
        let e = ConfigError::Parse("unexpected token".into());
        assert_eq!(PreflightIssueKind::from_config_error(&e), PreflightIssueKind::LoadError);
    }

    #[test]
    fn test_from_config_error_validation_error_returns_validation_error() {
        let e = ConfigError::Validation { section: "auth".into(), reason: "missing cert".into() };
        assert_eq!(PreflightIssueKind::from_config_error(&e), PreflightIssueKind::ValidationError);
    }

    #[test]
    fn test_from_config_error_not_found_maps_to_load_error() {
        let e = ConfigError::NotFound("/etc/app".into());
        assert_eq!(PreflightIssueKind::from_config_error(&e), PreflightIssueKind::LoadError);
    }
}
