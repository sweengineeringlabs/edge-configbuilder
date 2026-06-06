//! Public concrete path validator returned by `create_validator`.

use crate::api::error::config_error::ConfigError;
use crate::api::validator::traits::validator_ops::ValidatorOps;

/// A ready-to-use path validator produced by [`ConfigLoaderFactory::create_validator`].
///
/// Validates that a filesystem path is suitable for use as a config directory:
/// it must either not exist (absent dirs are skipped at load time) or be an
/// actual directory — regular files and other non-directory entries are rejected.
///
/// Call `validate_path` directly — the [`Validator`] trait does not need to be
/// in scope.
///
/// [`ConfigLoaderFactory::create_validator`]: crate::ConfigLoaderFactory::create_validator
/// [`Validator`]: crate::Validator
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use swe_edge_configbuilder::ConfigLoaderFactory;
///
/// let validator = ConfigLoaderFactory::create_validator();
///
/// // Non-existent path is allowed (will be skipped at load time).
/// assert!(validator.validate_path(Path::new("/tmp/does-not-exist-abc")).is_ok());
///
/// // A regular file is rejected.
/// // Create a temp file and pass its path to see Err(ConfigError::Io).
/// ```
pub struct PathValidatorImpl {
    pub(crate) ops: Box<dyn ValidatorOps>,
}

impl PathValidatorImpl {
    /// Returns `Ok(())` when `target` is a valid config path, `Err` otherwise.
    pub fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        self.ops.check_path(target)
    }
}

impl crate::api::validator::traits::validator::Validator for PathValidatorImpl {
    fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        self.ops.check_path(target)
    }
}
