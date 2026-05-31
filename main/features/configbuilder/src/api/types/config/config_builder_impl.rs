//! Public concrete config builder returned by `create_config_builder`.

use std::path::PathBuf;

use crate::api::traits::config::config_builder::ConfigBuilder;

/// Concrete config builder returned by [`ConfigLoaderFactory::create_config_builder`].
///
/// This is the **only type from which you can call `build_loader()`** to finalise
/// configuration into a [`SectionLoaderImpl`].  The `build_loader` method is an
/// inherent method added by an extension impl in `saf/` (not on the [`ConfigBuilder`]
/// trait) so that this declaration in `api/` carries no dependency on `core/`.
///
/// # Usage
///
/// ```rust,ignore
/// use swe_edge_configbuilder::ConfigLoaderFactory;
///
/// let loader = ConfigLoaderFactory::create_config_builder()
///     .with_name(env!("CARGO_PKG_NAME"))
///     .with_version(env!("CARGO_PKG_VERSION"))
///     .build_loader()?;
///
/// let cfg: MyConfig = loader.load_section("my_section")?;
/// ```
///
/// # Why not `impl ConfigBuilder`?
///
/// SAF `create_config_builder()` functions return this concrete type, not
/// `impl ConfigBuilder`.  Returning the opaque trait type would prevent callers
/// from ever calling `build_loader()`, because `build_loader` is not part of the
/// [`ConfigBuilder`] trait contract.
///
/// [`ConfigLoaderFactory::create_config_builder`]: crate::api::types::config::ConfigLoaderFactory::create_config_builder
/// [`SectionLoaderImpl`]: crate::api::types::section_loader_impl::SectionLoaderImpl
pub struct ConfigBuilderImpl {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) config_dirs: Vec<PathBuf>,
}

impl ConfigBuilderImpl {
    /// Create an empty builder with no name, version, or config dirs set.
    ///
    /// Call [`with_name`] and [`with_version`] to seed the builder before finalising
    /// with [`build_loader`].  Prefer this over [`ConfigLoaderFactory::create_config_builder`]
    /// when constructing from within a crate that knows its own name at compile time.
    ///
    /// [`with_name`]: Self::with_name
    /// [`with_version`]: Self::with_version
    /// [`build_loader`]: Self::build_loader
    pub fn new() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            config_dirs: Vec::new(),
        }
    }

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

impl Default for ConfigBuilderImpl {
    fn default() -> Self {
        Self::new()
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
