use crate::api::loader::types::raw_feature::RawFeature;
use crate::api::ConfigError;

/// Type-erased view of a config source used internally by [`crate::api::loader::types::section_loader_impl::SectionLoaderImpl`].
///
/// Non-generic so it can be boxed as a trait object. All deserialization of
/// the returned `toml::Value` is performed by the caller.
pub trait LoaderOps: Send + Sync {
    /// Load the raw merged TOML value at `key` from all configured directories.
    fn load_section_value(&self, key: &str) -> Result<toml::Value, ConfigError>;

    /// Validate all configured directories.
    fn validate_dirs(&self) -> Result<(), ConfigError>;

    /// Load the raw feature data for `key`.
    fn load_feature_raw(&self, key: &str) -> Result<RawFeature, ConfigError>;
}
