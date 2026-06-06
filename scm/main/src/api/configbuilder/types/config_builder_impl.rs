//! Public concrete config builder returned by `create_config_builder`.

use std::path::PathBuf;
use std::time::Duration;

use crate::api::configbuilder::traits::config_builder::ConfigBuilder;

/// Concrete config builder returned by [`ConfigLoaderFactory::create_config_builder`].
///
/// This is the **only type from which you can call `build_loader()`** to finalise
/// configuration into a [`SectionLoaderImpl`].  The `build_loader` method is an
/// inherent method added by an extension impl in `saf/` (not on the [`ConfigBuilder`]
/// trait) so that this declaration in `api/` carries no dependency on `core/`.
///
/// Chain the fluent setters to configure XDG resolution, then call `build_loader()`
/// to get a [`SectionLoaderImpl`] ready to call `load_section` on.
///
/// # Why not `impl ConfigBuilder`?
///
/// SAF `create_config_builder()` functions return this concrete type, not
/// `impl ConfigBuilder`.  Returning the opaque trait type would prevent callers
/// from ever calling `build_loader()`, because `build_loader` is not part of the
/// [`ConfigBuilder`] trait contract.
///
/// [`ConfigLoaderFactory::create_config_builder`]: crate::ConfigLoaderFactory::create_config_builder
/// [`SectionLoaderImpl`]: crate::SectionLoaderImpl
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_configbuilder::ConfigBuilderImpl;
///
/// #[derive(serde::Deserialize, Default)]
/// struct AuthConfig { token_ttl_secs: u64 }
///
/// let loader = ConfigBuilderImpl::new()
///     .with_name(env!("CARGO_PKG_NAME"))
///     .with_version(env!("CARGO_PKG_VERSION"))
///     .with_config_dir("config/")
///     .build_loader()
///     .expect("config dir must be readable");
///
/// let cfg: AuthConfig = loader.load_section("auth").expect("auth section required");
/// ```
pub struct ConfigBuilderImpl {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) config_dirs: Vec<PathBuf>,
    pub(crate) read_timeout: Option<Duration>,
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::ConfigBuilderImpl;
    /// let b = ConfigBuilderImpl::new();
    /// assert!(b.name().is_empty());
    /// assert!(b.version().is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            config_dirs: Vec::new(),
            read_timeout: None,
        }
    }

    /// Return the configured application name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::ConfigBuilderImpl;
    /// let b = ConfigBuilderImpl::new().with_name("my-app");
    /// assert_eq!(b.name(), "my-app");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the configured application version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::ConfigBuilderImpl;
    /// let b = ConfigBuilderImpl::new().with_version("1.2.3");
    /// assert_eq!(b.version(), "1.2.3");
    /// ```
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Set the application name; used by `build_loader` to resolve XDG config paths.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::ConfigBuilderImpl;
    /// let b = ConfigBuilderImpl::new().with_name("swe-edge");
    /// assert_eq!(b.name(), "swe-edge");
    /// ```
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the application version string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::ConfigBuilderImpl;
    /// let b = ConfigBuilderImpl::new().with_version("0.1.0");
    /// assert_eq!(b.version(), "0.1.0");
    /// ```
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Append an explicit config directory; takes precedence over XDG resolution.
    ///
    /// Multiple calls accumulate directories in the order they are added — a key
    /// present in a later directory wins over one in an earlier directory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::ConfigBuilderImpl;
    /// // Adding two directories; the second wins on conflicts.
    /// let _b = ConfigBuilderImpl::new()
    ///     .with_config_dir("/etc/my-app")
    ///     .with_config_dir("/run/secrets/my-app");
    /// ```
    pub fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config_dirs.push(dir.into());
        self
    }

    /// Override the default 30-second read deadline for each `application.toml`.
    ///
    /// Use a tight deadline (e.g. `Duration::from_millis(500)`) in test harnesses
    /// to make stalled-filesystem scenarios fail fast.  In production the default
    /// 30 s is recommended — it covers slow spinning disks without hanging startup
    /// indefinitely on a stalled NFS/FUSE mount.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use swe_edge_configbuilder::ConfigBuilderImpl;
    ///
    /// let builder = ConfigBuilderImpl::new()
    ///     .with_read_timeout(Duration::from_secs(5));
    /// ```
    pub fn with_read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = Some(timeout);
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
