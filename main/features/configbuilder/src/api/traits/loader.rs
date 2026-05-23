use crate::api::error::config_error::ConfigError;

/// Load typed TOML sections from a layered config chain.
///
/// Implementations merge config directories in order (later wins) and return
/// `Ok(T::default())` when the requested key is absent from every source.
///
/// Obtain a concrete instance via the `saf/` factory functions.
pub trait Loader {
    /// Load the section at `key` (dotted path, e.g. `"outer.inner"`) from all
    /// configured directories.
    fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default;

    /// Validate that all configured directories are accessible.
    ///
    /// Non-existent paths are permitted; a path that exists but is not a
    /// directory is an error.
    fn validate(&self) -> Result<(), ConfigError>;
}
