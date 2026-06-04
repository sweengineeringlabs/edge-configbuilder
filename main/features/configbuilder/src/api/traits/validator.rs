use crate::api::error::config_error::ConfigError;
use std::path::Path;

/// Validates a filesystem path before it is used as a config directory.
///
/// The concrete implementation is [`PathValidatorImpl`], obtained from
/// [`ConfigLoaderFactory::create_validator`].  Import this trait only when
/// writing generic code that accepts any validator.
///
/// [`PathValidatorImpl`]: crate::PathValidatorImpl
/// [`ConfigLoaderFactory::create_validator`]: crate::ConfigLoaderFactory::create_validator
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use swe_edge_configbuilder::{ConfigLoaderFactory, Validator};
///
/// fn check(v: &dyn Validator, path: &str) {
///     match v.validate_path(Path::new(path)) {
///         Ok(()) => println!("{path}: ok"),
///         Err(e) => eprintln!("{path}: {e}"),
///     }
/// }
///
/// let validator = ConfigLoaderFactory::create_validator();
/// check(&validator, "/etc/my-app");
/// ```
pub trait Validator {
    /// Returns `Ok(())` when `target` is a valid config directory path, `Err` otherwise.
    ///
    /// A non-existent path is considered valid (it will be skipped at load time).
    /// A path that exists but is not a directory returns [`ConfigError::Io`].
    ///
    /// [`ConfigError::Io`]: crate::ConfigError::Io
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::Path;
    /// use swe_edge_configbuilder::{ConfigLoaderFactory, Validator};
    ///
    /// let v = ConfigLoaderFactory::create_validator();
    /// // Non-existent paths are always valid.
    /// assert!(v.validate_path(Path::new("/tmp/no-such-dir-xyzzy")).is_ok());
    /// ```
    fn validate_path(&self, target: &Path) -> Result<(), ConfigError>;
}
