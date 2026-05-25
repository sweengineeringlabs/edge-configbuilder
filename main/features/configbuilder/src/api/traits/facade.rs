use crate::api::error::config_error::ConfigError;
use std::path::PathBuf;

/// Internal trait for section loading.
/// Consumers should use the public SectionLoaderImpl concrete type, not this trait.
#[allow(dead_code)]
pub(crate) trait SectionLoaderSvc {
    /// Load the section at `key` from all configured directories.
    fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default;

    /// Validate that all configured directories are accessible.
    fn validate(&self) -> Result<(), ConfigError>;
}

/// Internal trait for path validation.
/// Consumers should use the public PathValidatorImpl concrete type, not this trait.
#[allow(dead_code)]
pub(crate) trait PathValidatorSvc {
    /// Returns `Ok(())` when `target` is a valid config path, `Err` otherwise.
    fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError>;
}

/// Internal trait for config building.
/// Consumers should use the public ConfigBuilderImpl concrete type, not this trait.
#[allow(dead_code)]
pub(crate) trait ConfigBuilderSvc {
    /// Return the configured application name.
    fn name(&self) -> &str;

    /// Return the configured application version.
    fn version(&self) -> &str;

    /// Set the application name; used by `build_loader` to resolve XDG paths.
    fn with_name(self, name: impl Into<String>) -> Self;

    /// Set the application version string.
    fn with_version(self, version: impl Into<String>) -> Self;

    /// Append an explicit config directory; takes precedence over XDG resolution.
    fn with_config_dir(self, dir: impl Into<PathBuf>) -> Self;

    /// Consume the builder and return a ready-to-use section loader.
    fn build_loader(self) -> Result<impl crate::api::traits::loader::Loader, ConfigError>;
}
