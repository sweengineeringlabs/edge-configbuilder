use std::path::PathBuf;

use crate::api::error::config_error::ConfigError;
use crate::api::traits::config::config_builder::ConfigBuilder;
use crate::api::traits::feature_loader::FeatureLoader;
use crate::api::traits::loader::Loader;
use crate::api::traits::substitution_policy::SubstitutionPolicy;
use crate::core::DefaultConfigBuilder;
use crate::saf::section::SectionLoaderImpl;

pub(crate) struct ConfigBuilderImplWithSubstitution {
    pub(crate) inner: DefaultConfigBuilder,
    pub(crate) policy: Box<dyn SubstitutionPolicy>,
}

impl ConfigBuilder for ConfigBuilderImplWithSubstitution {
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
        let mut loader = self.inner.build_loader_internal()?;
        loader.substitution_policy = Some(self.policy);
        Ok(SectionLoaderImpl { inner: loader })
    }
}
