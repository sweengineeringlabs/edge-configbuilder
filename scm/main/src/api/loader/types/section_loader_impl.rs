//! Public concrete section loader returned by the `saf/` factory functions.

use crate::api::loader::traits::loader_ops::LoaderOps;
use crate::api::loader::types::feature_state::FeatureState;
use crate::api::loader::types::loaded_feature::LoadedFeature;
use crate::api::ConfigError;

/// A ready-to-use section loader produced by the `create_loader*` and
/// `ConfigBuilderImpl::build_loader` factory functions.
///
/// Call the inherent methods directly — the [`Loader`] and [`FeatureLoader`]
/// traits do not need to be in scope for basic use. Import the traits only when
/// you need to pass the loader to a generic function bounded on `Loader` or
/// `FeatureLoader`.
///
/// [`Loader`]: crate::Loader
/// [`FeatureLoader`]: crate::FeatureLoader
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_configbuilder::ConfigLoaderFactory;
///
/// #[derive(serde::Deserialize, Default)]
/// struct TlsConfig { cert_path: String, key_path: String }
///
/// let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
///
/// // Load a mandatory section — returns T::default() when key is absent.
/// let tls: TlsConfig = loader.load_section("tls").expect("readable TOML");
///
/// // Validate that all configured directories are accessible.
/// loader.validate().expect("config dir must exist or be absent, not a file");
/// ```
pub struct SectionLoaderImpl {
    pub(crate) ops: Box<dyn LoaderOps>,
}

impl SectionLoaderImpl {
    /// Load the section at `key` (dotted path, e.g. `"outer.inner"`) from all
    /// configured directories, merging them with last-wins semantics.
    ///
    /// Returns `T::default()` when the section key is absent from every file.
    /// If the loader was created with a substitution policy, `{{VAR_NAME}}`
    /// placeholders will be substituted with environment variable values after
    /// loading.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::ConfigLoaderFactory;
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct AuthConfig { token_ttl_secs: u64 }
    ///
    /// let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
    /// let cfg: AuthConfig = loader.load_section("auth").unwrap();
    /// ```
    pub fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let val = self.ops.load_section_value(key)?;
        // Empty table == section absent from all files but files were found;
        // return the type default (same contract as DefaultSectionLoader).
        if val.as_table().is_some_and(|t| t.is_empty()) {
            return Ok(T::default());
        }
        val.try_into()
            .map_err(|e: toml::de::Error| ConfigError::Parse(e.to_string()))
    }

    /// Validate that all configured directories are accessible.
    ///
    /// Non-existent paths are permitted (they are skipped at load time); a path
    /// that exists but is not a directory is a hard error.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::ConfigLoaderFactory;
    /// let loader = ConfigLoaderFactory::create_loader_for_dir("/etc/my-app");
    /// loader.validate().expect("/etc/my-app must be a directory if it exists");
    /// ```
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.ops.validate_dirs()
    }

    /// Load the section at `key` with full metadata.
    ///
    /// Applies the env-var override → `enabled = false` → section presence precedence
    /// chain and records the reason in the returned [`LoadedFeature::record`].
    /// Use this when feeding results into a [`FeatureRegistry`] startup report.
    /// For simple ad-hoc loading, prefer [`load_optional_section`].
    ///
    /// [`FeatureRegistry`]: crate::FeatureRegistry
    /// [`load_optional_section`]: SectionLoaderImpl::load_optional_section
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{ConfigLoaderFactory, FeatureState};
    ///
    /// #[derive(serde::Deserialize)]
    /// struct CacheConfig { ttl_secs: u64 }
    ///
    /// let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
    /// let loaded = loader.load_feature::<CacheConfig>("cache").unwrap();
    /// println!("enabled: {}", loaded.state.is_enabled());
    /// println!("section: {}", loaded.record.section_name);
    /// ```
    pub fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        let raw = self.ops.load_feature_raw(key)?;
        let state = match raw.value {
            None => FeatureState::Disabled,
            Some(val) => FeatureState::Enabled(
                val.try_into()
                    .map_err(|e: toml::de::Error| ConfigError::Parse(e.to_string()))?,
            ),
        };
        Ok(LoadedFeature {
            state,
            record: raw.record,
        })
    }

    /// Load the section at `key` as a `FeatureState`, without metadata.
    ///
    /// A convenience wrapper over [`load_feature`] that discards the metadata
    /// record.  Use this for one-off loading; use [`load_feature`] or
    /// [`FeatureRegistry`] when you need observability or graceful degradation.
    ///
    /// [`load_feature`]: SectionLoaderImpl::load_feature
    /// [`FeatureRegistry`]: crate::FeatureRegistry
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{ConfigLoaderFactory, FeatureState};
    ///
    /// #[derive(serde::Deserialize)]
    /// struct CacheConfig { ttl_secs: u64 }
    ///
    /// let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
    /// match loader.load_optional_section::<CacheConfig>("cache").unwrap() {
    ///     FeatureState::Enabled(cfg) => println!("cache ttl: {}s", cfg.ttl_secs),
    ///     FeatureState::Disabled     => println!("cache disabled"),
    /// }
    /// ```
    pub fn load_optional_section<T>(&self, key: &str) -> Result<FeatureState<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.load_feature(key).map(|loaded| loaded.state)
    }
}

impl crate::api::loader::traits::loader::Loader for SectionLoaderImpl {
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

impl crate::api::loader::traits::feature_loader::FeatureLoader for SectionLoaderImpl {
    fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.load_feature(key)
    }
}
