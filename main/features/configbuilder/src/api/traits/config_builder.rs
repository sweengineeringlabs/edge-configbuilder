use crate::api::error::config_error::ConfigError;
use crate::api::traits::loader::Loader;
use std::path::PathBuf;

/// Assemble application configuration from named sources.
///
/// The `Sized` bound allows implementors to use `Self` in fluent builder
/// returns. Obtain a concrete instance via the `saf/` factory functions —
/// callers never name the concrete type.
pub(crate) trait ConfigBuilder: Sized {
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

    /// Consume the builder and return a ready-to-use loader.
    ///
    /// Resolution order (first match wins):
    /// 1. Explicit dirs added via [`with_config_dir`] — used verbatim.
    /// 2. App name set via [`with_name`] — resolved through the XDG chain.
    /// 3. No name, no dirs — falls back to `SWE_EDGE_CONFIG_DIR` or `config/`.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any configured directory path points to a
    /// file rather than a directory, or if an environment-variable-supplied path
    /// contains `..` traversal components.
    #[allow(dead_code)]
    fn build_loader(self) -> Result<impl Loader, ConfigError>;
}
