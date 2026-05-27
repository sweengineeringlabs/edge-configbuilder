use std::path::PathBuf;

use std::fmt;

use crate::api::feature::traits::feature_loader::FeatureLoader;
use crate::api::loader::errors::config_error::ConfigError;
use crate::api::loader::traits::config_builder::ConfigBuilder;
use crate::api::loader::traits::loader::Loader;
use crate::api::loader::traits::substitution_policy::SubstitutionPolicy;
use crate::api::loader::traits::validator::Validator;
use crate::api::types::feature::feature_record::FeatureRecord;
use crate::api::types::feature::feature_state::FeatureState;
use crate::api::types::feature::loaded_feature::LoadedFeature;
use crate::api::types::feature::on_error::OnError;
use crate::api::types::feature::override_source::OverrideSource;
use crate::core::{DefaultConfigBuilder, DefaultSectionLoader, DefaultValidator};
use crate::spi::OptionalSection;

/// Public facade for loading typed TOML sections from config directories.
///
/// Optionally supports environment variable substitution ({{VAR_NAME}} syntax)
/// if created with a substitution policy.
pub struct SectionLoaderImpl {
    inner: DefaultSectionLoader,
}

impl SectionLoaderImpl {
    /// Load the section at `key` (dotted path, e.g. `"outer.inner"`) from all configured directories.
    pub fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        use crate::api::loader::traits::loader::Loader;
        self.inner.load_section(key)
    }

    /// Validate that all configured directories are accessible.
    pub fn validate(&self) -> Result<(), ConfigError> {
        use crate::api::loader::traits::loader::Loader;
        self.inner.validate()
    }
}

impl Loader for SectionLoaderImpl {
    fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        SectionLoaderImpl::load_section(self, key)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        SectionLoaderImpl::validate(self)
    }
}

impl FeatureLoader for SectionLoaderImpl {
    fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.inner.load_feature(key)
    }
}

/// Load the section at `key` as an optional feature, returning `Disabled` when absent.
///
/// Presence of the section in any config file enables the feature; absence
/// disables it without raising an error.  Use [`OptionalSection::load_optional`]
/// when the section type also needs cross-field validation.
///
/// # Errors
///
/// Returns [`ConfigError::Io`] for unreadable files or size-limit violations,
/// and [`ConfigError::Parse`] for malformed TOML or deserialisation failures.
///
/// [`OptionalSection::load_optional`]: crate::spi::OptionalSection::load_optional
pub fn load_feature_section<T>(
    loader: &impl FeatureLoader,
    key: &str,
) -> Result<FeatureState<T>, ConfigError>
where
    T: serde::de::DeserializeOwned,
{
    loader.load_optional_section(key)
}

/// Public facade for path validation.
pub struct PathValidatorImpl;

impl PathValidatorImpl {
    /// Returns `Ok(())` when `target` is a valid config path, `Err` otherwise.
    pub fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        DefaultValidator.validate_path(target)
    }
}

impl Validator for PathValidatorImpl {
    fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        PathValidatorImpl::validate_path(self, target)
    }
}

/// Public facade for building application configuration.
pub struct ConfigBuilderImpl {
    inner: DefaultConfigBuilder,
}

impl ConfigBuilderImpl {
    /// Return the configured application name.
    pub fn name(&self) -> &str {
        use crate::api::loader::traits::config_builder::ConfigBuilder;
        self.inner.name()
    }

    /// Return the configured application version.
    pub fn version(&self) -> &str {
        use crate::api::loader::traits::config_builder::ConfigBuilder;
        self.inner.version()
    }

    /// Set the application name; used by `build_loader` to resolve XDG paths.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        use crate::api::loader::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_name(name);
        self
    }

    /// Set the application version string.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        use crate::api::loader::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_version(version);
        self
    }

    /// Append an explicit config directory; takes precedence over XDG resolution.
    pub fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        use crate::api::loader::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_config_dir(dir);
        self
    }

    /// Consume the builder and return a ready-to-use section loader.
    pub fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        Ok(SectionLoaderImpl {
            inner: self.inner.build_loader_internal()?,
        })
    }
}

impl ConfigBuilder for ConfigBuilderImpl {
    fn name(&self) -> &str {
        ConfigBuilderImpl::name(self)
    }

    fn version(&self) -> &str {
        ConfigBuilderImpl::version(self)
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

/// Create a loader reading from `SWE_EDGE_CONFIG_DIR`, falling back to `config/`.
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if `SWE_EDGE_CONFIG_DIR` contains `..` traversal
/// components, or if the resolved path exists but is not a directory.
pub fn create_loader() -> Result<SectionLoaderImpl, ConfigError> {
    let loader = DefaultConfigBuilder {
        name: String::new(),
        version: String::new(),
        config_dirs: Vec::new(),
    }
    .build_loader_internal()?;
    Ok(SectionLoaderImpl { inner: loader })
}

/// Create a loader reading from a single explicit directory.
pub fn create_loader_for_dir(dir: impl Into<PathBuf>) -> SectionLoaderImpl {
    SectionLoaderImpl {
        inner: DefaultSectionLoader {
            config_dirs: vec![dir.into()],
            substitution_policy: None,
        },
    }
}

/// Create a loader following the XDG Base Directory chain for `app_name`.
///
/// Layer order (last wins):
/// - `$XDG_CONFIG_DIRS/<app_name>/` (lowest priority)
/// - `$XDG_CONFIG_HOME/<app_name>/`
/// - `$SWE_EDGE_CONFIG_DIR/` (if set)
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if any environment-variable-supplied path
/// contains `..` traversal components, or if a resolved path exists but is
/// not a directory.
pub fn create_loader_xdg(app_name: &str) -> Result<SectionLoaderImpl, ConfigError> {
    let loader = DefaultConfigBuilder {
        name: app_name.to_owned(),
        version: String::new(),
        config_dirs: Vec::new(),
    }
    .build_loader_internal()?;
    Ok(SectionLoaderImpl { inner: loader })
}

/// Create a path validator.
pub fn create_validator() -> PathValidatorImpl {
    PathValidatorImpl
}

/// Create a config builder pre-seeded with this package's name and version.
///
/// Uses XDG Base Directory resolution for the package name so callers do not
/// need to call the builder methods manually.
pub fn create_config_builder() -> ConfigBuilderImpl {
    ConfigBuilderImpl {
        inner: DefaultConfigBuilder {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_dirs: Vec::new(),
        },
    }
}

// ============================================================================
// Factory functions with substitution policy support
// ============================================================================

/// Create a loader with environment variable substitution support.
///
/// Loads config from `SWE_EDGE_CONFIG_DIR` (if set) or `config/`, with substitution
/// of `{{VAR_NAME}}` placeholders in TOML values using the provided policy.
///
/// # Arguments
/// * `policy` - Validation policy defining which env vars can be substituted
///   - Use [`AllowAllPolicy`] for testing (unsafe for production)
///   - Use [`PrefixWhitelistPolicy`] to restrict to prefixes like `["APP_", "DB_"]`
///   - Use [`PatternWhitelistPolicy`] for regex-based validation
///   - Use [`CompositePolicy`] to layer multiple policies
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if `SWE_EDGE_CONFIG_DIR` contains `..` traversal
/// components, or if the resolved path exists but is not a directory.
/// During [`load_section`](crate::api::loader::traits::loader::Loader::load_section),
/// returns error if an environment variable is missing or rejected by policy.
///
/// # Example
/// ```ignore
/// use swe_edge_configbuilder::{create_loader_with_substitution, PrefixWhitelistPolicy};
///
/// let policy = PrefixWhitelistPolicy::new(vec!["APP_".into(), "DB_".into()]);
/// let loader = create_loader_with_substitution(Box::new(policy))?;
///
/// // TOML: [section]
/// //       url = "postgresql://{{DB_HOST}}:{{DB_PORT}}/mydb"
/// let config: MyConfig = loader.load_section("section")?;
/// ```
pub fn create_loader_with_substitution(
    policy: Box<dyn SubstitutionPolicy>,
) -> Result<SectionLoaderImpl, ConfigError> {
    let mut loader = DefaultConfigBuilder {
        name: String::new(),
        version: String::new(),
        config_dirs: Vec::new(),
    }
    .build_loader_internal()?;
    loader.substitution_policy = Some(policy);
    Ok(SectionLoaderImpl { inner: loader })
}

/// Create a loader from a single directory with substitution support.
///
/// # Arguments
/// * `dir` - Explicit config directory (does not use XDG resolution)
/// * `policy` - Environment variable validation policy
///
/// # Example
/// ```ignore
/// use swe_edge_configbuilder::{create_loader_for_dir_with_substitution, PrefixWhitelistPolicy};
///
/// let policy = PrefixWhitelistPolicy::new(vec!["APP_".into()]);
/// let loader = create_loader_for_dir_with_substitution("/etc/myapp", Box::new(policy));
/// ```
pub fn create_loader_for_dir_with_substitution(
    dir: impl Into<PathBuf>,
    policy: Box<dyn SubstitutionPolicy>,
) -> SectionLoaderImpl {
    SectionLoaderImpl {
        inner: DefaultSectionLoader {
            config_dirs: vec![dir.into()],
            substitution_policy: Some(policy),
        },
    }
}

/// Create an XDG-aware loader with substitution support.
///
/// Follows the XDG Base Directory standard, searching paths in order:
/// 1. `$XDG_CONFIG_DIRS/<app_name>/` (lowest priority)
/// 2. `$XDG_CONFIG_HOME/<app_name>/`
/// 3. `$SWE_EDGE_CONFIG_DIR/` (if set)
///
/// Later directories override earlier ones via recursive table merge.
///
/// # Arguments
/// * `app_name` - Application name for XDG path resolution
/// * `policy` - Environment variable validation policy
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if any environment-variable-supplied path
/// contains `..` traversal components, or if a resolved path exists but is
/// not a directory.
///
/// # Example
/// ```ignore
/// use swe_edge_configbuilder::{create_loader_xdg_with_substitution, PrefixWhitelistPolicy};
///
/// let policy = PrefixWhitelistPolicy::new(vec!["APP_".into()]);
/// let loader = create_loader_xdg_with_substitution("myapp", Box::new(policy))?;
/// // Searches: ~/.config/myapp/application.toml, /etc/xdg/myapp/application.toml, etc.
/// ```
pub fn create_loader_xdg_with_substitution(
    app_name: &str,
    policy: Box<dyn SubstitutionPolicy>,
) -> Result<SectionLoaderImpl, ConfigError> {
    let mut loader = DefaultConfigBuilder {
        name: app_name.to_owned(),
        version: String::new(),
        config_dirs: Vec::new(),
    }
    .build_loader_internal()?;
    loader.substitution_policy = Some(policy);
    Ok(SectionLoaderImpl { inner: loader })
}

// ============================================================================
// FeatureRegistry — startup feature collector
// ============================================================================

type FeatureObserver = Box<dyn Fn(&FeatureRecord)>;

/// Collects feature-load metadata at startup for all optional TOML sections.
///
/// Call [`FeatureRegistry::load`] once per feature during application startup.
/// After loading all features, call [`FeatureRegistry::summary`] to obtain a
/// [`FeatureSummary`] suitable for log output.
///
/// Register observability callbacks via [`FeatureRegistry::on_load`] to emit
/// metrics or traces after each feature is resolved.
///
/// # Example
///
/// ```rust,ignore
/// use swe_edge_configbuilder::{FeatureRegistry, OptionalSection};
///
/// let mut registry = FeatureRegistry::new();
/// registry.on_load(|r| tracing::info!(section = r.section_name, enabled = r.enabled));
/// let broker = registry.load::<MessageBrokerConfig>(&loader)?;
/// let cache  = registry.load::<CacheConfig>(&loader)?;
///
/// tracing::info!("{}", registry.summary());
/// ```
pub struct FeatureRegistry {
    records: Vec<FeatureRecord>,
    observers: Vec<FeatureObserver>,
}

impl Default for FeatureRegistry {
    fn default() -> Self {
        Self::new()
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

    /// Register a callback invoked once per feature immediately after it is
    /// committed to the registry.
    ///
    /// The callback receives a shared reference to the [`FeatureRecord`] that
    /// was just stored.  Multiple observers are called in registration order.
    /// Callbacks are not invoked when [`load`] returns `Err` (hard failures).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// registry.on_load(|r| {
    ///     metrics::counter!("feature.loaded", 1, "section" => r.section_name.clone());
    /// });
    /// ```
    ///
    /// [`load`]: FeatureRegistry::load
    pub fn on_load(&mut self, observer: impl Fn(&FeatureRecord) + 'static) {
        self.observers.push(Box::new(observer));
    }

    /// Load an optional section, apply graceful-degradation policy if validation
    /// fails, and record the outcome for the startup summary.
    ///
    /// Applies `on_error` policy when `validate_enabled` rejects a section:
    /// - [`OnError::Fail`] — propagates the error; startup halts.
    /// - [`OnError::Disable`] — records the feature as disabled with
    ///   [`OverrideSource::ValidationError`] and continues startup.
    ///
    /// The env var `SWE_EDGE_FEATURE_<UPPER_KEY>_ON_ERROR=fail|disable` overrides
    /// the trait default.
    ///
    /// # Returns
    ///
    /// - `Ok(Enabled(T))` — section present, all controls say on, validation passed.
    /// - `Ok(Disabled)` — section absent, env var says off, `enabled = false`, or
    ///   validation failed with `on_error = Disable`.
    /// - `Err` — I/O error, parse failure, or `validate_enabled` rejection with
    ///   `on_error = Fail`.
    pub fn load<T>(&mut self, loader: &impl FeatureLoader) -> Result<FeatureState<T>, ConfigError>
    where
        T: OptionalSection,
    {
        let loaded: LoadedFeature<T> = loader.load_feature(T::section_name())?;

        let validation_result = if let FeatureState::Enabled(ref value) = loaded.state {
            Some(value.validate_enabled())
        } else {
            None
        };

        let (final_state, final_override) = match validation_result {
            Some(Ok(())) | None => (loaded.state, loaded.record.override_source),
            Some(Err(e)) => match resolve_on_error::<T>(T::section_name()) {
                OnError::Fail => return Err(e),
                OnError::Disable => (
                    FeatureState::Disabled,
                    Some(OverrideSource::ValidationError {
                        reason: e.to_string(),
                    }),
                ),
            },
        };

        self.records.push(FeatureRecord {
            section_name: loaded.record.section_name,
            enabled: final_state.is_enabled(),
            override_source: final_override,
            requires: T::requires(),
            metadata: T::metadata(),
        });

        if let Some(record) = self.records.last() {
            for observer in &self.observers {
                observer(record);
            }
        }

        Ok(final_state)
    }

    /// Check that every enabled feature's declared dependencies are also enabled.
    ///
    /// Call this after all features have been loaded.  Reports every violation
    /// in a single error so operators can fix all dependency issues in one pass.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Validation`] listing every unsatisfied dependency.
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
            Err(ConfigError::validation(
                "feature_dependencies",
                violations.join("; "),
            ))
        }
    }

    /// All feature records collected so far, in load order.
    pub fn records(&self) -> &[FeatureRecord] {
        &self.records
    }

    /// Produce a startup summary of every registered feature.
    pub fn summary(&self) -> FeatureSummary {
        FeatureSummary {
            records: self.records.clone(),
        }
    }
}

// ============================================================================
// FeatureSummary — human-readable startup report
// ============================================================================

/// A point-in-time snapshot of every feature loaded through [`FeatureRegistry`].
///
/// Implements [`Display`] so you can log it directly: `tracing::info!("{}", summary)`.
///
/// [`Display`]: std::fmt::Display
pub struct FeatureSummary {
    records: Vec<FeatureRecord>,
}

impl FeatureSummary {
    /// Number of features that resolved to enabled.
    pub fn enabled_count(&self) -> usize {
        self.records.iter().filter(|r| r.enabled).count()
    }

    /// Number of features that resolved to disabled.
    pub fn disabled_count(&self) -> usize {
        self.records.iter().filter(|r| !r.enabled).count()
    }

    /// Total number of registered features.
    pub fn total_count(&self) -> usize {
        self.records.len()
    }

    /// Whether every registered feature resolved to enabled.
    pub fn all_enabled(&self) -> bool {
        self.records.iter().all(|r| r.enabled)
    }
}

impl fmt::Display for FeatureSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "features: {}/{} enabled",
            self.enabled_count(),
            self.total_count()
        )?;
        for record in &self.records {
            let status = if record.enabled { "ON " } else { "OFF" };
            let override_note = match &record.override_source {
                None => String::new(),
                Some(OverrideSource::ExplicitTomlFlag) => " [disabled by enabled=false]".to_owned(),
                Some(OverrideSource::EnvVar { var_name, value }) => {
                    format!(" [env {var_name}={value}]")
                }
                Some(OverrideSource::ValidationError { reason }) => {
                    format!(" [DEGRADED: {reason}]")
                }
            };
            let description = if record.metadata.description.is_empty() {
                String::new()
            } else {
                format!("  — {}", record.metadata.description)
            };
            let owner = if record.metadata.owner.is_empty() {
                String::new()
            } else {
                format!(" (owner: {})", record.metadata.owner)
            };
            let deprecated = match record.metadata.deprecated_since {
                None => String::new(),
                Some(v) => format!(" [DEPRECATED since {v}]"),
            };
            writeln!(
                f,
                "  [{status}] {}{override_note}{description}{owner}{deprecated}",
                record.section_name
            )?;
        }
        Ok(())
    }
}

fn resolve_on_error<T: OptionalSection>(key: &str) -> OnError {
    let var_name = format!(
        "SWE_EDGE_FEATURE_{}_ON_ERROR",
        key.to_uppercase().replace('.', "_")
    );
    match std::env::var(&var_name).as_deref() {
        Ok("disable") => OnError::Disable,
        Ok("fail") => OnError::Fail,
        _ => T::on_error(),
    }
}

// ============================================================================
// Factory functions with substitution policy support
// ============================================================================

/// Create a config builder that supports substitution and custom paths.
///
/// Returns a builder pre-seeded with the calling package's name and version,
/// allowing for flexible configuration of paths and substitution policy.
/// Use the builder methods to customize paths, then call [`ConfigBuilderImplWithSubstitution::build_loader`]
/// to create the loader.
///
/// # Arguments
/// * `policy` - Validation policy for environment variable substitution
///
/// # Example
/// ```ignore
/// use swe_edge_configbuilder::{create_config_builder_with_substitution, PrefixWhitelistPolicy};
///
/// let policy = PrefixWhitelistPolicy::new(vec!["SERVICE_".into(), "APP_".into()]);
/// let loader = create_config_builder_with_substitution(Box::new(policy))
///     .with_config_dir("/etc/myapp")
///     .build_loader()?;
/// ```
pub fn create_config_builder_with_substitution(
    policy: Box<dyn SubstitutionPolicy>,
) -> ConfigBuilderImplWithSubstitution {
    ConfigBuilderImplWithSubstitution {
        inner: DefaultConfigBuilder {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_dirs: Vec::new(),
        },
        policy,
    }
}

/// Config builder with substitution policy support.
pub struct ConfigBuilderImplWithSubstitution {
    inner: DefaultConfigBuilder,
    policy: Box<dyn SubstitutionPolicy>,
}

impl ConfigBuilderImplWithSubstitution {
    /// Return the configured application name.
    pub fn name(&self) -> &str {
        use crate::api::loader::traits::config_builder::ConfigBuilder;
        self.inner.name()
    }

    /// Return the configured application version.
    pub fn version(&self) -> &str {
        use crate::api::loader::traits::config_builder::ConfigBuilder;
        self.inner.version()
    }

    /// Set the application name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        use crate::api::loader::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_name(name);
        self
    }

    /// Set the application version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        use crate::api::loader::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_version(version);
        self
    }

    /// Add a config directory.
    pub fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        use crate::api::loader::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_config_dir(dir);
        self
    }

    /// Build the loader with substitution support.
    pub fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        let mut loader = self.inner.build_loader_internal()?;
        loader.substitution_policy = Some(self.policy);
        Ok(SectionLoaderImpl { inner: loader })
    }
}

impl ConfigBuilder for ConfigBuilderImplWithSubstitution {
    fn name(&self) -> &str {
        ConfigBuilderImplWithSubstitution::name(self)
    }

    fn version(&self) -> &str {
        ConfigBuilderImplWithSubstitution::version(self)
    }

    fn with_name(self, name: impl Into<String>) -> Self {
        ConfigBuilderImplWithSubstitution::with_name(self, name)
    }

    fn with_version(self, version: impl Into<String>) -> Self {
        ConfigBuilderImplWithSubstitution::with_version(self, version)
    }

    fn with_config_dir(self, dir: impl Into<PathBuf>) -> Self {
        ConfigBuilderImplWithSubstitution::with_config_dir(self, dir)
    }
}
