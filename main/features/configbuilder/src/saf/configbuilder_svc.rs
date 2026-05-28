use std::path::PathBuf;

use crate::api::error::config_error::ConfigError;
use crate::api::traits::config::config_builder::ConfigBuilder;
use crate::api::traits::feature_loader::FeatureLoader;
use crate::api::traits::loader::Loader;
use crate::api::traits::substitution_policy::SubstitutionPolicy;
use crate::api::traits::validator::Validator;
use crate::api::types::feature::feature_state::FeatureState;
use crate::core::{DefaultConfigBuilder, DefaultSectionLoader};
use crate::saf::config::ConfigBuilderImpl;
use crate::saf::path::PathValidatorImpl;
use crate::saf::section::SectionLoaderImpl;
use crate::saf::substitution::ConfigBuilderImplWithSubstitution;

/// Create a loader reading from `SWE_EDGE_CONFIG_DIR`, falling back to `config/`.
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if `SWE_EDGE_CONFIG_DIR` contains `..` traversal
/// components, or if the resolved path exists but is not a directory.
pub fn create_loader() -> Result<impl Loader + FeatureLoader, ConfigError> {
    let loader = DefaultConfigBuilder {
        name: String::new(),
        version: String::new(),
        config_dirs: Vec::new(),
    }
    .build_loader_internal()?;
    Ok(SectionLoaderImpl { inner: loader })
}

/// Create a loader reading from a single explicit directory.
pub fn create_loader_for_dir(dir: impl Into<PathBuf>) -> impl Loader + FeatureLoader {
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
pub fn create_loader_xdg(app_name: &str) -> Result<impl Loader + FeatureLoader, ConfigError> {
    let loader = DefaultConfigBuilder {
        name: app_name.to_owned(),
        version: String::new(),
        config_dirs: Vec::new(),
    }
    .build_loader_internal()?;
    Ok(SectionLoaderImpl { inner: loader })
}

/// Create a path validator.
pub fn create_validator() -> impl Validator {
    PathValidatorImpl
}

/// Create a config builder pre-seeded with this package's name and version.
///
/// Uses XDG Base Directory resolution for the package name so callers do not
/// need to call the builder methods manually.
pub fn create_config_builder() -> impl ConfigBuilder {
    ConfigBuilderImpl {
        inner: DefaultConfigBuilder {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_dirs: Vec::new(),
        },
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
/// [`OptionalSection::load_optional`]: crate::api::traits::optional_section::OptionalSection::load_optional
pub fn load_feature_section<T>(
    loader: &impl FeatureLoader,
    key: &str,
) -> Result<FeatureState<T>, ConfigError>
where
    T: serde::de::DeserializeOwned,
{
    loader.load_optional_section(key)
}

/// Create a loader with environment variable substitution support.
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if `SWE_EDGE_CONFIG_DIR` contains `..` traversal
/// components, or if the resolved path exists but is not a directory.
pub fn create_loader_with_substitution(
    policy: Box<dyn SubstitutionPolicy>,
) -> Result<impl Loader + FeatureLoader, ConfigError> {
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
pub fn create_loader_for_dir_with_substitution(
    dir: impl Into<PathBuf>,
    policy: Box<dyn SubstitutionPolicy>,
) -> impl Loader + FeatureLoader {
    SectionLoaderImpl {
        inner: DefaultSectionLoader {
            config_dirs: vec![dir.into()],
            substitution_policy: Some(policy),
        },
    }
}

/// Create an XDG-aware loader with substitution support.
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if any environment-variable-supplied path
/// contains `..` traversal components, or if a resolved path exists but is
/// not a directory.
pub fn create_loader_xdg_with_substitution(
    app_name: &str,
    policy: Box<dyn SubstitutionPolicy>,
) -> Result<impl Loader + FeatureLoader, ConfigError> {
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
/// Returns a builder pre-seeded with the calling package's name and version.
pub fn create_config_builder_with_substitution(
    policy: Box<dyn SubstitutionPolicy>,
) -> impl ConfigBuilder {
    ConfigBuilderImplWithSubstitution {
        inner: DefaultConfigBuilder {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_dirs: Vec::new(),
        },
        policy,
    }
}
