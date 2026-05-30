//! Public concrete config builder returned by `create_config_builder`.

use std::path::PathBuf;

use crate::api::traits::config::config_builder::ConfigBuilder;

/// A ready-to-use config builder produced by `create_config_builder`.
///
/// Use the fluent builder methods to configure directories, then call
/// `build_loader` to obtain a [`SectionLoaderImpl`].
///
/// The `build_loader` method is provided by an extension impl in `saf/` so
/// that this type carries no dependency on `core/` (SEA rules 46 and 116).
///
/// [`SectionLoaderImpl`]: crate::api::types::section_loader_impl::SectionLoaderImpl
pub struct ConfigBuilderImpl {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) config_dirs: Vec<PathBuf>,
}

impl ConfigBuilderImpl {
    /// Return the configured application name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the configured application version.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Set the application name; used by `build_loader` to resolve XDG paths.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the application version string.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Append an explicit config directory; takes precedence over XDG resolution.
    ///
    /// Multiple calls accumulate directories — later entries win on key conflicts.
    pub fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config_dirs.push(dir.into());
        self
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
