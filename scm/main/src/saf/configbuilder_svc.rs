use std::path::PathBuf;
use std::time::Duration;

use crate::{
    CompositePolicy, ConfigBuilderImpl, ConfigError, ConfigLoaderFactory, FeatureMetadata,
    FeatureRecord, FeatureRecordBuilder, FeatureRegistry, FeatureState, FeatureSummary,
    LoadedFeature, OnError, OptionalSection, OverrideSource, PathValidatorImpl,
    PatternWhitelistPolicy, PrefixWhitelistPolicy, PreflightIssue, PreflightIssueKind,
    PreflightReport, SectionLoaderImpl, SubstitutionConfigBuilderImpl, SubstitutionPolicy,
    Topology,
};

// ── Extension impls for the builder types ────────────────────────────────────
//
// These impls live in saf/ so that the api/types/ files carry no dependency on
// core/ (SEA rules 46 and 116).  The structs in api/types/ store only primitive
// data; saf/ wires them to the concrete DefaultConfigBuilder at call time.

impl ConfigBuilderImpl {
    /// Create an empty builder with no name, version, or config dirs set.
    ///
    /// Call [`with_name`] and [`with_version`] to seed the builder before finalising
    /// with [`build_loader`].  Prefer this over [`ConfigLoaderFactory::create_config_builder`](crate::ConfigLoaderFactory::create_config_builder)
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
            read_timeout: None,
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

    /// Set the application name; used by `build_loader` to resolve XDG config paths.
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
    pub fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config_dirs.push(dir.into());
        self
    }

    /// Override the default 30-second read deadline for each `application.toml`.
    pub fn with_read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = Some(timeout);
        self
    }
}

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
    pub fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config_dirs.push(dir.into());
        self
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
    ///         Box::new(ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
    ///             "APP_".to_string()
    ///         ])),
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
    fn touch_core_api() {
        let enabled = FeatureState::Enabled(1_u8);
        let _ = enabled.is_enabled();
        let _ = FeatureState::<u8>::Disabled.is_disabled();
        let _ = FeatureState::Enabled(1_u8).into_option();
        let _ = FeatureState::Enabled(1_u8).as_option();
        let _ = FeatureState::Enabled(1_u8).map(|n| n + 1);
        let _ = FeatureState::Enabled(1_u8).and_then(|n| FeatureState::Enabled(n + 1));
        let _ = FeatureState::Enabled(1_u8).unwrap_or(0);
        let _ = FeatureState::<u8>::Disabled.unwrap_or_else(|| 0);
        let _ = FeatureState::<u8>::Disabled.enabled_or_default();

        let summary = FeatureSummary { records: vec![] };
        let _ = summary.enabled_count();
        let _ = summary.disabled_count();
        let _ = summary.total_count();
        let _ = summary.all_enabled();

        let _ = Topology::sort(&["a"], &[&[]]);

        let _ = PreflightIssueKind::from_config_error(&ConfigError::Parse(String::new()));
        let mut report = Self::create_preflight_report();
        report.push(PreflightIssue {
            section: String::from("touch_section"),
            kind: PreflightIssueKind::LoadError,
            message: String::from("touch"),
        });
        let _ = report.is_ok();
        let _ = report.issues();
        let _ = report.issue_count();

        let _ = Self::create_prefix_whitelist_policy(vec!["APP_".to_string()]).prefixes();
        if let Ok(p) = Self::create_pattern_whitelist_policy(r"^APP_[A-Z_]+$".to_string()) {
            let _ = p.pattern().len();
        }
        let _ = Self::create_composite_policy(vec![]);
        let _ = PathValidatorImpl {
            ops: Box::new(crate::core::DefaultValidator),
        }
        .validate_path(std::path::Path::new("."));

        let _ = FeatureRecordBuilder::new("touch")
            .enabled(false)
            .override_source(OverrideSource::ExplicitTomlFlag)
            .requires(&[])
            .metadata(FeatureMetadata::default())
            .build();
    }

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
        Self::touch_core_api();
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

    /// Create an empty preflight report.
    pub fn create_preflight_report() -> PreflightReport {
        PreflightReport { issues: Vec::new() }
    }

    /// Inspect whether a preflight report has no issues.
    pub fn preflight_report_is_ok(report: &PreflightReport) -> bool {
        report.is_ok()
    }

    /// Append an issue to a preflight report.
    pub fn preflight_report_push(report: &mut PreflightReport, issue: PreflightIssue) {
        report.push(issue);
    }

    /// Return the issues contained in a preflight report.
    pub fn preflight_report_issues(report: &PreflightReport) -> &[PreflightIssue] {
        report.issues()
    }

    /// Return the number of issues contained in a preflight report.
    pub fn preflight_report_issue_count(report: &PreflightReport) -> usize {
        report.issue_count()
    }

    /// Create a prefix-whitelist substitution policy.
    pub fn create_prefix_whitelist_policy(prefixes: Vec<String>) -> PrefixWhitelistPolicy {
        PrefixWhitelistPolicy { prefixes }
    }

    /// Create a regex-based substitution policy.
    pub fn create_pattern_whitelist_policy(
        pattern: String,
    ) -> Result<PatternWhitelistPolicy, String> {
        PatternWhitelistPolicy::new(pattern)
    }

    /// Create a composite substitution policy.
    pub fn create_composite_policy(policies: Vec<Box<dyn SubstitutionPolicy>>) -> CompositePolicy {
        CompositePolicy { policies }
    }

    /// Create an empty feature registry.
    pub fn create_feature_registry() -> FeatureRegistry {
        FeatureRegistry::new()
    }

    /// Load one optional section into a feature registry.
    pub fn feature_registry_load<T>(
        registry: &mut FeatureRegistry,
        loader: &SectionLoaderImpl,
    ) -> Result<FeatureState<T>, ConfigError>
    where
        T: OptionalSection,
    {
        registry.load(loader)
    }

    /// Return the records tracked by a feature registry.
    pub fn feature_registry_records(registry: &FeatureRegistry) -> &[crate::FeatureRecord] {
        registry.records()
    }

    /// Return a summary snapshot for a feature registry.
    pub fn feature_registry_summary(registry: &FeatureRegistry) -> FeatureSummary {
        registry.summary()
    }

    /// Validate the dependency graph recorded in a feature registry.
    pub fn feature_registry_validate_dependencies(
        registry: &FeatureRegistry,
    ) -> Result<(), ConfigError> {
        registry.validate_dependencies()
    }

    /// Register an observer that runs after each successful registry load.
    pub fn feature_registry_on_load(
        registry: &mut FeatureRegistry,
        observer: impl Fn(&crate::FeatureRecord) + 'static,
    ) {
        registry.on_load(observer)
    }

    /// Sort feature names according to their dependency declarations.
    pub fn topology_sort(names: &[&str], requires: &[&[&str]]) -> Result<Vec<usize>, String> {
        Topology::sort(names, requires)
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
    ///         Box::new(ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
    ///             "APP_".to_string()
    ///         ])),
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
    ///     Box::new(ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
    ///         "APP_".to_string()
    ///     ])),
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
    ///         Box::new(ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
    ///             "APP_".to_string()
    ///         ])),
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
    ///         Box::new(ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
    ///             "APP_".to_string()
    ///         ])),
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

impl<T> FeatureState<T> {
    /// Return `true` when the state holds an enabled value.
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled(_))
    }

    /// Return `true` when the state is disabled.
    pub fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Convert into an `Option<T>`, discarding disabled states.
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Enabled(v) => Some(v),
            Self::Disabled => None,
        }
    }

    /// Borrow the inner value when enabled.
    pub fn as_option(&self) -> Option<&T> {
        match self {
            Self::Enabled(v) => Some(v),
            Self::Disabled => None,
        }
    }

    /// Map the inner value when enabled.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> FeatureState<U> {
        match self {
            Self::Enabled(v) => FeatureState::Enabled(f(v)),
            Self::Disabled => FeatureState::Disabled,
        }
    }

    /// Chain another state-producing operation when enabled.
    pub fn and_then<U>(self, f: impl FnOnce(T) -> FeatureState<U>) -> FeatureState<U> {
        match self {
            Self::Enabled(v) => f(v),
            Self::Disabled => FeatureState::Disabled,
        }
    }

    /// Return the inner value or the provided default when disabled.
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Enabled(v) => v,
            Self::Disabled => default,
        }
    }

    /// Return the inner value or compute one lazily when disabled.
    pub fn unwrap_or_else(self, f: impl FnOnce() -> T) -> T {
        match self {
            Self::Enabled(v) => v,
            Self::Disabled => f(),
        }
    }

    /// Return the enabled value or `T::default()` when disabled.
    pub fn enabled_or_default(self) -> T
    where
        T: Default,
    {
        self.unwrap_or_else(T::default)
    }
}

impl FeatureSummary {
    /// Count enabled feature records.
    pub fn enabled_count(&self) -> usize {
        self.records.iter().filter(|r| r.enabled).count()
    }

    /// Count disabled feature records.
    pub fn disabled_count(&self) -> usize {
        self.records.iter().filter(|r| !r.enabled).count()
    }

    /// Count total feature records.
    pub fn total_count(&self) -> usize {
        self.records.len()
    }

    /// Return `true` when all records are enabled.
    pub fn all_enabled(&self) -> bool {
        self.records.iter().all(|r| r.enabled)
    }
}

impl FeatureRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            observers: Vec::new(),
        }
    }

    /// Register a callback to observe loaded records.
    pub fn on_load(&mut self, observer: impl Fn(&FeatureRecord) + 'static) {
        self.observers.push(Box::new(observer));
    }

    /// Load a feature section and record its state.
    pub fn load<T>(&mut self, loader: &SectionLoaderImpl) -> Result<FeatureState<T>, ConfigError>
    where
        T: OptionalSection,
    {
        let loaded: LoadedFeature<T> = loader.load_feature(T::section_name())?;
        let LoadedFeature { state, record } = loaded;
        let record = *record;

        let validation_result = if let FeatureState::Enabled(ref value) = state {
            Some(value.validate_enabled())
        } else {
            None
        };

        let (final_state, final_override) = match validation_result {
            Some(Ok(())) | None => (state, record.override_source),
            Some(Err(e)) => match crate::core::DefaultSectionLoader::resolve_feature_on_error::<T>(
                T::section_name(),
            ) {
                OnError::Fail => return Err(e),
                OnError::Disable => (
                    FeatureState::Disabled,
                    Some(OverrideSource::ValidationError {
                        reason: e.to_string(),
                    }),
                ),
            },
        };

        let mut built = FeatureRecordBuilder::new(record.section_name)
            .enabled(final_state.is_enabled())
            .requires(T::requires())
            .metadata(T::metadata());
        if let Some(override_source) = final_override {
            built = built.override_source(override_source);
        }
        self.records.push(built.build());

        if let Some(record) = self.records.last() {
            for observer in &self.observers {
                observer(record);
            }
        }

        Ok(final_state)
    }

    /// Validate recorded dependencies.
    pub fn validate_dependencies(&self) -> Result<(), ConfigError> {
        let enabled: std::collections::HashSet<&str> = self
            .records
            .iter()
            .filter(|r| r.enabled)
            .map(|r| r.section_name.as_str())
            .collect();

        let violations: Vec<String> = self
            .records
            .iter()
            .filter(|r| r.enabled)
            .flat_map(|r| {
                r.requires.iter().filter_map(|dep| {
                    if enabled.contains(dep) {
                        None
                    } else {
                        Some(format!(
                            "'{}' requires '{}' but '{}' is not enabled",
                            r.section_name, dep, dep
                        ))
                    }
                })
            })
            .collect();

        if violations.is_empty() {
            Ok(())
        } else {
            Err(ConfigError::Validation {
                section: String::from("feature_dependencies"),
                reason: violations.join("; "),
            })
        }
    }

    /// Borrow the recorded feature records.
    pub fn records(&self) -> &[FeatureRecord] {
        &self.records
    }

    /// Build a snapshot summary of the recorded features.
    pub fn summary(&self) -> FeatureSummary {
        FeatureSummary {
            records: self.records.clone(),
        }
    }
}

impl FeatureRecordBuilder {
    /// Create a new record builder for the named section.
    pub fn new(section_name: impl Into<String>) -> Self {
        Self {
            section_name: section_name.into(),
            enabled: false,
            override_source: None,
            requires: &[],
            metadata: Box::new(FeatureMetadata::default()),
        }
    }

    /// Mark the feature as enabled or disabled.
    pub fn enabled(mut self, v: bool) -> Self {
        self.enabled = v;
        self
    }

    /// Record the source that overrode the feature state.
    pub fn override_source(mut self, v: OverrideSource) -> Self {
        self.override_source = Some(v);
        self
    }

    /// Attach the required feature dependencies.
    pub fn requires(mut self, v: &'static [&'static str]) -> Self {
        self.requires = v;
        self
    }

    /// Attach feature metadata to the record under construction.
    pub fn metadata(mut self, v: FeatureMetadata) -> Self {
        self.metadata = Box::new(v);
        self
    }

    /// Finalize the builder and return the feature record.
    pub fn build(self) -> FeatureRecord {
        FeatureRecord {
            section_name: self.section_name,
            enabled: self.enabled,
            override_source: self.override_source,
            requires: self.requires,
            metadata: self.metadata,
        }
    }
}

impl SectionLoaderImpl {
    /// Load and deserialize a named section.
    pub fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let val = self.ops.load_section_value(key)?;
        if val.as_table().is_some_and(|t| t.is_empty()) {
            return Ok(T::default());
        }
        val.try_into()
            .map_err(|e: toml::de::Error| ConfigError::Parse(e.to_string()))
    }

    /// Validate the loader's configured directories.
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.ops.validate_dirs()
    }

    /// Load a named feature and return its state plus record metadata.
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

    /// Load a named section and return only its enabled/disabled state.
    pub fn load_optional_section<T>(&self, key: &str) -> Result<FeatureState<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.load_feature(key).map(|loaded| loaded.state)
    }
}

impl Topology {
    /// Return a topological ordering of the provided names.
    pub fn sort(names: &[&str], requires: &[&[&str]]) -> Result<Vec<usize>, String> {
        let n = names.len();
        let index: std::collections::HashMap<&str, usize> = names
            .iter()
            .enumerate()
            .map(|(i, &name)| (name, i))
            .collect();

        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        let mut in_degree: Vec<usize> = vec![0; n];

        for (i, deps) in requires.iter().enumerate() {
            for dep in *deps {
                if let Some(&j) = index.get(dep) {
                    adj[j].push(i);
                    in_degree[i] += 1;
                }
            }
        }

        let mut queue: std::collections::VecDeque<usize> = in_degree
            .iter()
            .enumerate()
            .filter(|(_, &d)| d == 0)
            .map(|(i, _)| i)
            .collect();

        let mut order = Vec::with_capacity(n);
        while let Some(i) = queue.pop_front() {
            order.push(i);
            for &j in &adj[i] {
                in_degree[j] -= 1;
                if in_degree[j] == 0 {
                    queue.push_back(j);
                }
            }
        }

        if order.len() == n {
            Ok(order)
        } else {
            let cycle: Vec<&str> = names
                .iter()
                .enumerate()
                .filter(|(i, _)| in_degree[*i] > 0)
                .map(|(_, name)| *name)
                .collect();
            Err(format!(
                "dependency cycle detected among: {}",
                cycle.join(", ")
            ))
        }
    }
}

impl PreflightIssueKind {
    /// Map a config error to the matching preflight issue kind.
    pub fn from_config_error(e: &ConfigError) -> Self {
        match e {
            ConfigError::Parse(_) | ConfigError::Io(_) | ConfigError::NotFound(_) => {
                Self::LoadError
            }
            ConfigError::Validation { .. } => Self::ValidationError,
        }
    }
}

impl PreflightReport {
    /// Create an empty preflight report.
    pub fn new() -> Self {
        Self { issues: Vec::new() }
    }

    /// Add a preflight issue to the report.
    pub fn push(&mut self, issue: PreflightIssue) {
        self.issues.push(issue);
    }

    /// Return true when the report contains no issues.
    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    /// Borrow the collected preflight issues.
    pub fn issues(&self) -> &[PreflightIssue] {
        &self.issues
    }

    /// Return the number of collected issues.
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }
}

impl CompositePolicy {
    /// Create a composite policy from the supplied policy list.
    pub fn new(policies: Vec<Box<dyn SubstitutionPolicy>>) -> Self {
        Self { policies }
    }
}

impl PatternWhitelistPolicy {
    /// Create a regex-backed whitelist policy.
    pub fn new(pattern: String) -> Result<Self, String> {
        regex::Regex::new(&pattern)
            .map(|regex| Self {
                pattern: regex,
                pattern_str: pattern,
            })
            .map_err(|e| format!("Invalid regex pattern: {}", e))
    }

    /// Return the original regex pattern string.
    pub fn pattern(&self) -> &str {
        &self.pattern_str
    }
}

impl PrefixWhitelistPolicy {
    /// Create a prefix-based whitelist policy.
    pub fn new(prefixes: Vec<String>) -> Self {
        Self { prefixes }
    }

    /// Return the configured allowed prefixes.
    pub fn prefixes(&self) -> &[String] {
        &self.prefixes
    }
}

impl PathValidatorImpl {
    /// Validate a filesystem path using the configured validator.
    pub fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        self.ops.check_path(target).map_err(ConfigError::from)
    }
}
