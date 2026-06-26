//! Public concrete section loader returned by the `saf/` factory functions.

use crate::{ConfigError, FeatureLoader, LoadedFeature, Loader};

impl Loader for crate::SectionLoaderImpl {
    fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        self.load_section(key)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        self.validate()
    }
}

impl FeatureLoader for crate::SectionLoaderImpl {
    fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.load_feature(key)
    }
}
