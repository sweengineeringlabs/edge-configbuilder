use crate::api::validator::errors::validator_error::ValidatorError;

/// Type-erased path validation used internally by [`crate::api::validator::types::path_validator_impl::PathValidatorImpl`].
pub trait ValidatorOps: Send + Sync {
    /// Returns `Ok(())` when `target` is a valid config path, `Err` otherwise.
    fn check_path(&self, target: &std::path::Path) -> Result<(), ValidatorError>;
}
