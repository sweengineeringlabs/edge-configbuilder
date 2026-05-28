//! Public concrete substitution config builder returned by
//! `create_config_builder_with_substitution`.

use std::path::PathBuf;

use crate::api::traits::substitution_policy::SubstitutionPolicy;

/// A ready-to-use config builder with substitution support, produced by
/// `create_config_builder_with_substitution`.
///
/// Use the fluent builder methods to configure directories, then call
/// `build_loader` to obtain a [`SectionLoaderImpl`].
///
/// The `build_loader` method is provided by an extension impl in `saf/` so
/// that this type carries no dependency on `core/` (SEA rules 46 and 116).
///
/// [`SectionLoaderImpl`]: crate::api::types::section_loader_impl::SectionLoaderImpl
pub struct SubstitutionConfigBuilderImpl {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) config_dirs: Vec<PathBuf>,
    pub(crate) policy: Box<dyn SubstitutionPolicy>,
}

impl SubstitutionConfigBuilderImpl {
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
