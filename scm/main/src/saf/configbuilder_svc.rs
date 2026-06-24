use std::path::PathBuf;

use crate::api::{
    ConfigBuilderImpl, ConfigError, ConfigLoaderFactory, FeatureState, PathValidatorImpl,
    SectionLoaderImpl, SubstitutionConfigBuilderImpl, SubstitutionPolicy,
};

// ── Extension impls for the builder types ────────────────────────────────────
//
// These impls live in saf/ so that the api/types/ files carry no dependency on
// core/ (SEA rules 46 and 116).  The structs in api/types/ store only primitive
// data; saf/ wires them to the concrete DefaultConfigBuilder at call time.

impl ConfigBuilderImpl {
    /// Consume the builder and return a ready-to-use section loader.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any environment-variable-supplied path
    /// contains `..` traversal components, or if a resolved path exists but is
    /// not a directory.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::ConfigBuilderImpl;
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct AppConfig { log_level: String }
    ///
    /// let loader = ConfigBuilderImpl::new()
    ///     .with_name("my-app")
    ///     .with_config_dir("config/")
    ///     .build_loader()
    ///     .expect("config dir accessible");
    ///
    /// let cfg: AppConfig = loader.load_section("app").unwrap();
    /// ```
    pub fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        let core = crate::core::DefaultConfigBuilder {
            name: self.name,
            version: self.version,
            config_dirs: self.config_dirs,
            read_timeout: self
                .read_timeout
                .unwrap_or(crate::core::DEFAULT_READ_TIMEOUT),
        }
        .build_loader_internal()?;
        Ok(SectionLoaderImpl {
            ops: Box::new(core),
        })
    }
}

impl SubstitutionConfigBuilderImpl {
    /// Consume the builder and return a ready-to-use section loader with
    /// substitution support.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any environment-variable-supplied path
    /// contains `..` traversal components, or if a resolved path exists but is
    /// not a directory.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{PrefixWhitelistPolicy, ConfigLoaderFactory};
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct DbConfig { url: String }
    ///
    /// let loader = ConfigLoaderFactory::create_config_builder_with_substitution(
    ///         Box::new(PrefixWhitelistPolicy::new(vec!["APP_".to_string()])),
    ///     )
    ///     .with_config_dir("config/")
    ///     .build_loader()
    ///     .expect("config dir accessible");
    ///
    /// let cfg: DbConfig = loader.load_section("database").unwrap();
    /// ```
    pub fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        let mut core = crate::core::DefaultConfigBuilder {
            name: self.name,
            version: self.version,
            config_dirs: self.config_dirs,
            read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
        }
        .build_loader_internal()?;
        core.substitution_policy = Some(self.policy);
        Ok(SectionLoaderImpl {
            ops: Box::new(core),
        })
    }
}

// ── ConfigLoaderFactory — all factory fns as associated functions on a type ───

impl ConfigLoaderFactory {
    /// Create a loader reading from `SWE_EDGE_CONFIG_DIR`, falling back to `config/`.
    ///
    /// This is the **default entry point** for production services that read
    /// config from a single environment-controlled directory.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if `SWE_EDGE_CONFIG_DIR` contains `..` traversal
    /// components, or if the resolved path exists but is not a directory.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::ConfigLoaderFactory;
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct AppConfig { log_level: String }
    ///
    /// let loader = ConfigLoaderFactory::create_loader()
    ///     .expect("SWE_EDGE_CONFIG_DIR or config/ must be a directory if it exists");
    ///
    /// let cfg: AppConfig = loader.load_section("app").unwrap_or_default();
    /// ```
    pub fn create_loader() -> Result<SectionLoaderImpl, ConfigError> {
        let loader = crate::core::DefaultConfigBuilder {
            name: String::new(),
            version: String::new(),
            config_dirs: Vec::new(),
            read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
        }
        .build_loader_internal()?;
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    /// Create a loader reading from a single explicit directory.
    ///
    /// Useful in integration tests where the directory is known at call time.
    /// Does not read any environment variables.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::ConfigLoaderFactory;
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct TlsConfig { cert_path: String }
    ///
    /// let loader = ConfigLoaderFactory::create_loader_for_dir("tests/fixtures/config");
    /// let tls: TlsConfig = loader.load_section("tls").unwrap_or_default();
    /// ```
    pub fn create_loader_for_dir(dir: impl Into<PathBuf>) -> SectionLoaderImpl {
        SectionLoaderImpl {
            ops: Box::new(crate::core::DefaultSectionLoader {
                config_dirs: vec![dir.into()],
                substitution_policy: None,
                read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
            }),
        }
    }

    /// Create a loader following the XDG Base Directory chain for `app_name`.
    ///
    /// Layer order (last wins):
    /// - `$XDG_CONFIG_DIRS/<app_name>/` (lowest priority)
    /// - `$XDG_CONFIG_HOME/<app_name>/`
    /// - `$SWE_EDGE_CONFIG_DIR/` (if set, highest priority)
    ///
    /// Use this for user-facing CLI tools or desktop services that should respect
    /// XDG conventions. Prefer [`create_loader`] for server-side services.
    ///
    /// [`create_loader`]: ConfigLoaderFactory::create_loader
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any environment-variable-supplied path
    /// contains `..` traversal components, or if a resolved path exists but is
    /// not a directory.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::ConfigLoaderFactory;
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct AppConfig { theme: String }
    ///
    /// let loader = ConfigLoaderFactory::create_loader_xdg("my-cli-tool")
    ///     .expect("XDG dirs must be directories if they exist");
    /// let cfg: AppConfig = loader.load_section("ui").unwrap_or_default();
    /// ```
    pub fn create_loader_xdg(app_name: &str) -> Result<SectionLoaderImpl, ConfigError> {
        let loader = crate::core::DefaultConfigBuilder {
            name: app_name.to_owned(),
            version: String::new(),
            config_dirs: Vec::new(),
            read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
        }
        .build_loader_internal()?;
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    /// Create a path validator for checking config directory paths.
    ///
    /// Returns a [`PathValidatorImpl`] that accepts non-existent paths and
    /// rejects paths that exist but are not directories.
    ///
    /// [`PathValidatorImpl`]: crate::PathValidatorImpl
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::Path;
    /// use swe_edge_configbuilder::ConfigLoaderFactory;
    ///
    /// let validator = ConfigLoaderFactory::create_validator();
    /// assert!(validator.validate_path(Path::new("/tmp/absent-dir-xyz")).is_ok());
    /// ```
    pub fn create_validator() -> PathValidatorImpl {
        PathValidatorImpl {
            ops: Box::new(crate::core::DefaultValidator),
        }
    }

    /// Create a config builder pre-seeded with this package's name and version.
    ///
    /// Uses XDG Base Directory resolution for the package name so callers do not
    /// need to call the builder methods manually. Chain `.with_config_dir()` to
    /// add extra directories, then call `.build_loader()` to finalise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::ConfigLoaderFactory;
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct AuthConfig { token_ttl_secs: u64 }
    ///
    /// let loader = ConfigLoaderFactory::create_config_builder()
    ///     .build_loader()
    ///     .expect("config dir accessible");
    ///
    /// let cfg: AuthConfig = loader.load_section("auth").unwrap_or_default();
    /// ```
    pub fn create_config_builder() -> ConfigBuilderImpl {
        let mut b = ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Load the section at `key` as an optional feature, returning `Disabled` when absent.
    ///
    /// Presence of the section in any config file enables the feature; absence
    /// disables it without raising an error. For a full startup workflow with
    /// graceful degradation and dependency validation, prefer [`FeatureRegistry::load`].
    ///
    /// [`FeatureRegistry::load`]: crate::FeatureRegistry::load
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] for unreadable files or size-limit violations,
    /// and [`ConfigError::Parse`] for malformed TOML or deserialisation failures.
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
    /// match ConfigLoaderFactory::load_feature_section::<CacheConfig>(&loader, "cache").unwrap() {
    ///     FeatureState::Enabled(cfg) => println!("cache ttl: {}s", cfg.ttl_secs),
    ///     FeatureState::Disabled     => println!("cache not configured"),
    /// }
    /// ```
    pub fn load_feature_section<T>(
        loader: &SectionLoaderImpl,
        key: &str,
    ) -> Result<FeatureState<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        loader.load_optional_section(key)
    }

    /// Create a loader with environment variable substitution support.
    ///
    /// Reads from `SWE_EDGE_CONFIG_DIR` or `config/` (same as [`create_loader`])
    /// and expands `{{VAR_NAME}}` placeholders in TOML values using `policy`.
    ///
    /// [`create_loader`]: ConfigLoaderFactory::create_loader
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if `SWE_EDGE_CONFIG_DIR` contains `..` traversal
    /// components, or if the resolved path exists but is not a directory.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{PrefixWhitelistPolicy, ConfigLoaderFactory};
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct DbConfig { url: String }
    ///
    /// // TOML: url = "postgres://{{DB_USER}}@host/db"
    /// let loader = ConfigLoaderFactory::create_loader_with_substitution(
    ///         Box::new(PrefixWhitelistPolicy::new(vec!["APP_".to_string()])),
    ///     )
    ///     .expect("config dir accessible");
    ///
    /// let cfg: DbConfig = loader.load_section("database").unwrap_or_default();
    /// ```
    pub fn create_loader_with_substitution(
        policy: Box<dyn SubstitutionPolicy>,
    ) -> Result<SectionLoaderImpl, ConfigError> {
        let mut loader = crate::core::DefaultConfigBuilder {
            name: String::new(),
            version: String::new(),
            config_dirs: Vec::new(),
            read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
        }
        .build_loader_internal()?;
        loader.substitution_policy = Some(policy);
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    /// Create a loader from a single explicit directory with substitution support.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{PrefixWhitelistPolicy, ConfigLoaderFactory};
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct AppConfig { host: String }
    ///
    /// let loader = ConfigLoaderFactory::create_loader_for_dir_with_substitution(
    ///     "config/",
    ///     Box::new(PrefixWhitelistPolicy::new(vec!["APP_".to_string()])),
    /// );
    /// let cfg: AppConfig = loader.load_section("app").unwrap_or_default();
    /// ```
    pub fn create_loader_for_dir_with_substitution(
        dir: impl Into<PathBuf>,
        policy: Box<dyn SubstitutionPolicy>,
    ) -> SectionLoaderImpl {
        SectionLoaderImpl {
            ops: Box::new(crate::core::DefaultSectionLoader {
                config_dirs: vec![dir.into()],
                substitution_policy: Some(policy),
                read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
            }),
        }
    }

    /// Load section `key` from the XDG config chain for `app_name` in one call.
    ///
    /// Equivalent to `ConfigLoaderFactory::create_loader_xdg(app_name)?.load_section(key)`,
    /// provided so callers never need to hold an intermediate loader reference.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any XDG path contains `..` traversal components
    /// or is not a directory, and [`ConfigError::Parse`] if the section cannot be
    /// deserialised into `T`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::ConfigLoaderFactory;
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct GoalConfig { target: String }
    ///
    /// let cfg: GoalConfig = ConfigLoaderFactory::load_section_xdg("my-app", "goal")
    ///     .unwrap_or_default();
    /// ```
    pub fn load_section_xdg<T>(app_name: &str, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        ConfigLoaderFactory::create_loader_xdg(app_name)?.load_section(key)
    }

    /// Create an XDG-aware loader with substitution support.
    ///
    /// Combines XDG multi-directory resolution (same as [`create_loader_xdg`])
    /// with `{{VAR_NAME}}` substitution governed by `policy`.
    ///
    /// [`create_loader_xdg`]: ConfigLoaderFactory::create_loader_xdg
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Io`] if any environment-variable-supplied path
    /// contains `..` traversal components, or if a resolved path exists but is
    /// not a directory.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{PrefixWhitelistPolicy, ConfigLoaderFactory};
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct AppConfig { api_key: String }
    ///
    /// // TOML: api_key = "{{MY_APP_API_KEY}}"
    /// let loader = ConfigLoaderFactory::create_loader_xdg_with_substitution(
    ///         "my-app",
    ///         Box::new(PrefixWhitelistPolicy::new(vec!["APP_".to_string()])),
    ///     )
    ///     .expect("XDG dirs accessible");
    ///
    /// let cfg: AppConfig = loader.load_section("api").unwrap_or_default();
    /// ```
    pub fn create_loader_xdg_with_substitution(
        app_name: &str,
        policy: Box<dyn SubstitutionPolicy>,
    ) -> Result<SectionLoaderImpl, ConfigError> {
        let mut loader = crate::core::DefaultConfigBuilder {
            name: app_name.to_owned(),
            version: String::new(),
            config_dirs: Vec::new(),
            read_timeout: crate::core::DEFAULT_READ_TIMEOUT,
        }
        .build_loader_internal()?;
        loader.substitution_policy = Some(policy);
        Ok(SectionLoaderImpl {
            ops: Box::new(loader),
        })
    }

    /// Create a config builder pre-seeded with this package's name and version,
    /// with substitution support bound to `policy`.
    ///
    /// Chain `.with_config_dir()` to add directories, then call
    /// [`SubstitutionConfigBuilderImpl::build_loader`] to finalise.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_configbuilder::{PrefixWhitelistPolicy, ConfigLoaderFactory};
    ///
    /// #[derive(serde::Deserialize, Default)]
    /// struct DbConfig { url: String }
    ///
    /// let loader = ConfigLoaderFactory::create_config_builder_with_substitution(
    ///         Box::new(PrefixWhitelistPolicy::new(vec!["APP_".to_string()])),
    ///     )
    ///     .with_config_dir("config/")
    ///     .build_loader()
    ///     .expect("config dir accessible");
    ///
    /// let cfg: DbConfig = loader.load_section("database").unwrap_or_default();
    /// ```
    pub fn create_config_builder_with_substitution(
        policy: Box<dyn SubstitutionPolicy>,
    ) -> SubstitutionConfigBuilderImpl {
        SubstitutionConfigBuilderImpl {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_dirs: Vec::new(),
            policy,
        }
    }
}
