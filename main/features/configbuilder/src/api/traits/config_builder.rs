use std::path::PathBuf;

use crate::api::error::config_error::ConfigError;
use crate::api::traits::feature_loader::FeatureLoader;
use crate::api::traits::loader::Loader;

/// Assemble application configuration from named sources.
///
/// The `Sized` bound allows implementors to use `Self` in fluent builder
/// returns. Obtain a concrete instance via the `saf/` factory functions —
/// callers never name the concrete type. Call `build_loader()` on the
/// returned builder to finalise configuration.
pub trait ConfigBuilder: Sized {
    /// Return the configured application name.
    fn name(&self) -> &str;

    /// Return the configured application version.
    fn version(&self) -> &str;

    /// Set the application name; used by `build_loader` to resolve XDG paths.
    fn with_name(self, name: impl Into<String>) -> Self;

    /// Set the application version string.
    fn with_version(self, version: impl Into<String>) -> Self;

    /// Append an explicit config directory; takes precedence over XDG resolution.
    ///
    /// Multiple calls accumulate directories — later entries win on key conflicts.
    fn with_config_dir(self, dir: impl Into<PathBuf>) -> Self;

    /// Consume the builder and return a ready-to-use section loader.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any environment-variable-supplied path
    /// contains `..` traversal components, or if a resolved path exists but is
    /// not a directory.
    fn build_loader(self) -> Result<impl Loader + FeatureLoader, ConfigError>;
}
