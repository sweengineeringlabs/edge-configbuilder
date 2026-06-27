use std::path::PathBuf;
use std::time::Duration;

use crate::api::{ConfigBuilder, ConfigBuilderImpl, ConfigError, SectionLoaderImpl};

impl ConfigBuilderImpl {
    /// Create an empty builder with no name, version, or config dirs set.
    pub(crate) fn new() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            config_dirs: Vec::new(),
            read_timeout: None,
        }
    }

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

    /// Override the default 30-second read deadline.
    pub(crate) fn with_read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = Some(timeout);
        self
    }

    /// Consume the builder and return a ready-to-use section loader.
    pub(crate) fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        let core = super::DefaultConfigBuilder {
            name: self.name,
            version: self.version,
            config_dirs: self.config_dirs,
            read_timeout: self.read_timeout.unwrap_or(crate::core::loader::DEFAULT_READ_TIMEOUT),
        }.build_loader_internal()?;
        Ok(SectionLoaderImpl {
            ops: Box::new(core),
        })
    }
}

impl Default for ConfigBuilderImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::api::ConfigBuilderInit for ConfigBuilderImpl {
    fn new() -> Self {
        ConfigBuilderImpl::new()
    }

    fn with_read_timeout(self, timeout: Duration) -> Self {
        ConfigBuilderImpl::with_read_timeout(self, timeout)
    }
}

impl crate::api::BuilderFinalizer for ConfigBuilderImpl {
    type Loader = SectionLoaderImpl;
    type Error = ConfigError;

    fn build_loader(self) -> Result<Self::Loader, Self::Error> {
        ConfigBuilderImpl::build_loader(self)
    }
}

impl ConfigBuilder for ConfigBuilderImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn with_name(self, name: impl Into<String>) -> Self {
        ConfigBuilderImpl::with_name(self, name)
    }

    fn with_version(self, version: impl Into<String>) -> Self {
        ConfigBuilderImpl::with_version(self, version)
    }

    fn with_config_dir(self, dir: impl Into<PathBuf>) -> Self {
        ConfigBuilderImpl::with_config_dir(self, dir)
    }
}
