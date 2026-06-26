use crate::{ConfigError, ValidatorError};

impl From<ValidatorError> for ConfigError {
    fn from(value: ValidatorError) -> Self {
        match value {
            ValidatorError::Io(message) => Self::Io(message),
        }
    }
}
