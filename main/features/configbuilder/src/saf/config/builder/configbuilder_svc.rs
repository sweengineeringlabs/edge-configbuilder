use std::path::PathBuf;

use crate::api::error::config_error::ConfigError;
use crate::api::traits::config::config_builder::ConfigBuilder;
use crate::api::traits::feature_loader::FeatureLoader;
use crate::api::traits::loader::Loader;
use crate::core::DefaultConfigBuilder;
use crate::saf::section::SectionLoaderImpl;

pub(crate) struct ConfigBuilderImpl {
    pub(crate) inner: DefaultConfigBuilder,
}

impl ConfigBuilder for ConfigBuilderImpl {
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn version(&self) -> &str {
        self.inner.version()
    }

    fn with_name(mut self, name: impl Into<String>) -> Self {
        self.inner = self.inner.with_name(name);
        self
    }

    fn with_version(mut self, version: impl Into<String>) -> Self {
        self.inner = self.inner.with_version(version);
        self
    }

    fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.inner = self.inner.with_config_dir(dir);
        self
    }

    fn build_loader(self) -> Result<impl Loader + FeatureLoader, ConfigError> {
        Ok(SectionLoaderImpl {
            inner: self.inner.build_loader_internal()?,
        })
    }
}
