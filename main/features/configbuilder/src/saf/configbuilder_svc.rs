use std::path::PathBuf;

use crate::api::error::config_error::ConfigError;
use crate::api::traits::config_builder::ConfigBuilder;
use crate::api::traits::facade::{ConfigBuilderSvc, PathValidatorSvc, SectionLoaderSvc};
use crate::api::traits::loader::Loader;
use crate::api::traits::substitution_policy::SubstitutionPolicy;
use crate::api::traits::validator::Validator;
use crate::core::{DefaultConfigBuilder, DefaultSectionLoader, DefaultValidator};

/// Public facade for loading typed TOML sections from config directories.
///
/// Optionally supports environment variable substitution ({{VAR_NAME}} syntax)
/// if created with a substitution policy.
pub struct SectionLoaderImpl {
    inner: DefaultSectionLoader,
}

impl SectionLoaderImpl {
    /// Load the section at `key` (dotted path, e.g. `"outer.inner"`) from all configured directories.
    #[allow(dead_code)]
    pub fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        use crate::api::traits::loader::Loader;
        self.inner.load_section(key)
    }

    /// Validate that all configured directories are accessible.
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), ConfigError> {
        use crate::api::traits::loader::Loader;
        self.inner.validate()
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
impl SectionLoaderSvc for SectionLoaderImpl {
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

/// Public facade for path validation.
pub struct PathValidatorImpl;

impl PathValidatorImpl {
    /// Returns `Ok(())` when `target` is a valid config path, `Err` otherwise.
    #[allow(dead_code)]
    pub fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        DefaultValidator.validate_path(target)
    }
}

#[allow(dead_code)]
impl Validator for PathValidatorImpl {
    fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        PathValidatorImpl::validate_path(self, target)
    }
}

#[allow(dead_code)]
impl PathValidatorSvc for PathValidatorImpl {
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
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        use crate::api::traits::config_builder::ConfigBuilder;
        self.inner.name()
    }

    /// Return the configured application version.
    #[allow(dead_code)]
    pub fn version(&self) -> &str {
        use crate::api::traits::config_builder::ConfigBuilder;
        self.inner.version()
    }

    /// Set the application name; used by `build_loader` to resolve XDG paths.
    #[allow(dead_code)]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        use crate::api::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_name(name);
        self
    }

    /// Set the application version string.
    #[allow(dead_code)]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        use crate::api::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_version(version);
        self
    }

    /// Append an explicit config directory; takes precedence over XDG resolution.
    #[allow(dead_code)]
    pub fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        use crate::api::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_config_dir(dir);
        self
    }

    /// Consume the builder and return a ready-to-use section loader.
    #[allow(dead_code)]
    pub fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        Ok(SectionLoaderImpl {
            inner: self.inner.build_loader_internal()?,
        })
    }
}

#[allow(dead_code)]
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

    fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        ConfigBuilderImpl::build_loader(self)
    }
}

#[allow(dead_code)]
impl ConfigBuilderSvc for ConfigBuilderImpl {
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

    fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        ConfigBuilderImpl::build_loader(self)
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
/// During [`load_section`](crate::api::traits::loader::Loader::load_section),
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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

#[allow(dead_code)]
impl ConfigBuilderImplWithSubstitution {
    /// Return the configured application name.
    pub fn name(&self) -> &str {
        use crate::api::traits::config_builder::ConfigBuilder;
        self.inner.name()
    }

    /// Return the configured application version.
    pub fn version(&self) -> &str {
        use crate::api::traits::config_builder::ConfigBuilder;
        self.inner.version()
    }

    /// Set the application name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        use crate::api::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_name(name);
        self
    }

    /// Set the application version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        use crate::api::traits::config_builder::ConfigBuilder;
        self.inner = self.inner.with_version(version);
        self
    }

    /// Add a config directory.
    pub fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        use crate::api::traits::config_builder::ConfigBuilder;
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

#[allow(dead_code)]
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

    fn build_loader(self) -> Result<SectionLoaderImpl, ConfigError> {
        ConfigBuilderImplWithSubstitution::build_loader(self)
    }
}
