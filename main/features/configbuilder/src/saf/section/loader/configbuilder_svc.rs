use crate::api::error::config_error::ConfigError;
use crate::api::traits::feature_loader::FeatureLoader;
use crate::api::traits::loader::Loader;
use crate::api::types::feature::loaded_feature::LoadedFeature;
use crate::core::DefaultSectionLoader;

pub(crate) struct SectionLoaderImpl {
    pub(crate) inner: DefaultSectionLoader,
}

impl Loader for SectionLoaderImpl {
    fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        self.inner.load_section(key)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        self.inner.validate()
    }
}

impl FeatureLoader for SectionLoaderImpl {
    fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.inner.load_feature(key)
    }
}
