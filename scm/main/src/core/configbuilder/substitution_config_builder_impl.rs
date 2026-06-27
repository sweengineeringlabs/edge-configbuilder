use std::path::PathBuf;

use crate::api::{ConfigBuilder, ConfigBuilderImpl, ConfigError, SectionLoaderImpl, SubstitutionConfigBuilderImpl};

impl SubstitutionConfigBuilderImpl {
    /// Return the configured application name.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// Return the configured application version.
    pub(crate) fn version(&self) -> &str {
        &self.version
    }

    /// Set the application name.
    pub(crate) fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the application version string.
    pub(crate) fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Append an explicit config directory.
    pub(crate) fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config_dirs.push(dir.into());
        self
    }

    /// Consume the builder and return a ready-to-use section loader with substitution support.
    pub(crate) fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        let mut core = crate::core::DefaultConfigBuilder {
            name: self.name,
            version: self.version,
            config_dirs: self.config_dirs,
            read_timeout: crate::core::loader::DEFAULT_READ_TIMEOUT,
        }.build_loader_internal()?;
        core.substitution_policy = Some(self.policy);
        Ok(SectionLoaderImpl {
            ops: Box::new(core),
        })
    }
}

impl crate::api::BuilderFinalizer for SubstitutionConfigBuilderImpl {
    type Loader = SectionLoaderImpl;
    type Error = ConfigError;

    fn build_loader(self) -> Result<Self::Loader, Self::Error> {
        SubstitutionConfigBuilderImpl::build_loader(self)
    }
}

impl ConfigBuilder for SubstitutionConfigBuilderImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn with_name(self, name: impl Into<String>) -> Self {
        SubstitutionConfigBuilderImpl::with_name(self, name)
    }

    fn with_version(self, version: impl Into<String>) -> Self {
        SubstitutionConfigBuilderImpl::with_version(self, version)
    }

    fn with_config_dir(self, dir: impl Into<std::path::PathBuf>) -> Self {
        SubstitutionConfigBuilderImpl::with_config_dir(self, dir)
    }
}

impl From<SubstitutionConfigBuilderImpl> for ConfigBuilderImpl {
    fn from(value: SubstitutionConfigBuilderImpl) -> Self {
        ConfigBuilderImpl {
            name: value.name,
            version: value.version,
            config_dirs: value.config_dirs,
            read_timeout: None,
        }
    }
}
