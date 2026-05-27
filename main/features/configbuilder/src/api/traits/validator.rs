use crate::api::loader::errors::config_error::ConfigError;
use std::path::Path;

/// Validates a configuration value before it is used.
pub(crate) trait Validator {
    /// Returns `Ok(())` when `target` is a valid config path, `Err` otherwise.
    fn validate_path(&self, target: &Path) -> Result<(), ConfigError>;
}
