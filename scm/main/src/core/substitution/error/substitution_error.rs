use std::fmt;

use crate::api::SubstitutionError;

impl fmt::Display for SubstitutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubstitutionError::VariableNotFound { var_name, location } => write!(
                f,
                "Environment variable '{}' referenced in config (at {}) but not found",
                var_name, location
            ),
            SubstitutionError::VariableRejected {
                var_name,
                reason,
                policy,
            } => write!(
                f,
                "Environment variable '{}' not allowed by validation policy ({}): {}",
                var_name, policy, reason
            ),
            SubstitutionError::InvalidSubstitutionSyntax { value, reason } => {
                write!(f, "Invalid substitution syntax in '{}': {}", value, reason)
            }
        }
    }
}

impl std::error::Error for SubstitutionError {}
