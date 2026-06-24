use crate::api::loader::errors::config_error::ConfigError;
use thiserror::Error;

/// Errors returned by path validation ports.
#[derive(Debug, Error)]
pub enum ValidatorError {
    /// An I/O or filesystem constraint was violated.
    #[error("config io error: {0}")]
    Io(String),
}

impl From<ValidatorError> for ConfigError {
    fn from(value: ValidatorError) -> Self {
        match value {
            ValidatorError::Io(message) => Self::Io(message),
        }
    }
}
