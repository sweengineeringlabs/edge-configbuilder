use crate::api::validator::errors::validator_error::ValidatorError;
use std::path::Path;

/// Validates a filesystem path before it is used as a config directory.
///
/// The concrete implementation is [`PathValidatorImpl`].  Import this trait
/// only when writing generic code that accepts any validator.
///
/// [`PathValidatorImpl`]: crate::PathValidatorImpl
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use swe_edge_configbuilder::Validator;
///
/// fn check(v: &dyn Validator, path: &str) {
///     match v.validate_path(Path::new(path)) {
///         Ok(()) => println!("{path}: ok"),
///         Err(e) => eprintln!("{path}: {e}"),
///     }
/// }
///
/// # let validator: swe_edge_configbuilder::PathValidatorImpl = panic!();
/// check(&validator, "/etc/my-app");
/// ```
pub trait Validator {
    /// Returns `Ok(())` when `target` is a valid config directory path, `Err` otherwise.
    ///
    /// A non-existent path is considered valid (it will be skipped at load time).
    /// A path that exists but is not a directory returns [`ValidatorError::Io`].
    ///
    /// [`ValidatorError::Io`]: crate::ValidatorError::Io
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::Path;
    /// use swe_edge_configbuilder::Validator;
    ///
    /// # let v: swe_edge_configbuilder::PathValidatorImpl = panic!();
    /// // Non-existent paths are always valid.
    /// assert!(v.validate_path(Path::new("/tmp/no-such-dir-xyzzy")).is_ok());
    /// ```
    fn validate_path(&self, target: &Path) -> Result<(), ValidatorError>;
}
