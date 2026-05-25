use crate::api::error::config_error::ConfigError;

/// Load typed TOML sections from a layered config chain.
///
/// Implementations merge config directories in order (later wins) and return
/// `Ok(T::default())` when the requested key is absent from every source.
///
/// Environment variable substitution ({{VAR_NAME}} syntax) is optionally supported.
pub(crate) trait Loader {
    /// Load the section at `key` (dotted path, e.g. `"outer.inner"`) from all
    /// configured directories.
    ///
    /// If the loader was created with a substitution policy, {{VAR_NAME}} placeholders
    /// will be substituted with environment variable values after loading.
    fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default;

    /// Validate that all configured directories are accessible.
    ///
    /// Non-existent paths are permitted; a path that exists but is not a
    /// directory is an error.
    fn validate(&self) -> Result<(), ConfigError>;
}
