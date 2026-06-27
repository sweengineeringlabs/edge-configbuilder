use crate::{ConfigError, PathValidatorImpl, ValidatorError};

impl PathValidatorImpl {
    /// Validate a filesystem path using the configured validator.
    pub(crate) fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        self.ops.check_path(target).map_err(ConfigError::from)
    }
}

impl crate::Validator for PathValidatorImpl {
    fn validate_path(&self, target: &std::path::Path) -> Result<(), ValidatorError> {
        self.ops.check_path(target)
    }
}
