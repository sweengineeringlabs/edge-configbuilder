//! Public concrete path validator returned by `create_validator`.

use crate::api::error::config_error::ConfigError;
use crate::api::traits::validator_ops::ValidatorOps;

/// A ready-to-use path validator produced by `create_validator`.
///
/// Call `validate_path` directly — the [`Validator`] trait does not need to be
/// in scope.
///
/// [`Validator`]: crate::api::traits::validator::Validator
pub struct PathValidatorImpl {
    pub(crate) ops: Box<dyn ValidatorOps>,
}

impl PathValidatorImpl {
    /// Returns `Ok(())` when `target` is a valid config path, `Err` otherwise.
    pub fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        self.ops.check_path(target)
    }
}

impl crate::api::traits::validator::Validator for PathValidatorImpl {
    fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        self.ops.check_path(target)
    }
}
