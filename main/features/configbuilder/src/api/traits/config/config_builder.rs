use std::path::PathBuf;

/// Assemble application configuration from named sources.
///
/// Obtain a concrete instance via the `saf/` factory functions (`create_config_builder`,
/// `create_config_builder_with_substitution`). Call `build_loader()` on the returned
/// builder to finalise configuration.
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
}
