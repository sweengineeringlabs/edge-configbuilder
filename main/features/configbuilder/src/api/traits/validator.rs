use crate::api::error::config_error::ConfigError;
use std::path::Path;

/// Validates a configuration path before it is used.
pub trait Validator {
    /// Returns `Ok(())` when `target` is a valid config path, `Err` otherwise.
    fn validate_path(&self, target: &Path) -> Result<(), ConfigError>;
}
