//! Standalone, runtime-independent TOML section loader for swe-edge services.
//!
//! Provides XDG-aware, layered config section loading for any
//! `T: DeserializeOwned + Default`. Library crates can depend on this crate
//! directly without pulling in `swe-edge-runtime-main`.
//!
//! # Usage
//!
//! ```rust,ignore
//! use swe_edge_configbuilder::create_loader;
//!
//! #[derive(serde::Deserialize, Default)]
//! struct CompletionConfig { model: String, max_tokens: u32 }
//!
//! let cfg: CompletionConfig =
//!     create_loader()?.load_section("application.completion")?;
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod gateway;
mod saf;

pub use crate::api::traits::config::config_builder::ConfigBuilder;
pub use crate::api::traits::config::config_section::ConfigSection;
pub use crate::api::traits::feature_loader::FeatureLoader;
pub use crate::api::traits::loader::Loader;
pub use crate::api::traits::optional_section::OptionalSection;
pub use crate::api::traits::substitution_policy::SubstitutionPolicy;
pub use crate::api::traits::validator::Validator;
pub use crate::api::types::application_config::ApplicationConfig;
pub use crate::api::types::config_builder_impl::ConfigBuilderImpl;
pub use crate::api::types::feature::{
    FeatureMetadata, FeatureRecord, FeatureRecordBuilder, FeatureState, LoadedFeature, OnError,
    OverrideSource,
};
pub use crate::api::types::loader::{
    AllowAllPolicy, CompositePolicy, PatternWhitelistPolicy, PrefixWhitelistPolicy,
};
pub use crate::api::types::path_validator_impl::PathValidatorImpl;
pub use crate::api::types::preflight::{PreflightIssue, PreflightIssueKind, PreflightReport};
pub use crate::api::types::section_loader_impl::SectionLoaderImpl;
pub use crate::api::types::substitution_config_builder_impl::SubstitutionConfigBuilderImpl;
pub use saf::*;

// ── Backward-compatible module-level aliases ──────────────────────────────────
// These delegate to ConfigLoaderFactory so callers using the flat import style
// (swe_edge_configbuilder::create_loader()) continue to work unchanged.

/// Create a loader reading from `SWE_EDGE_CONFIG_DIR`, falling back to `config/`.
#[inline]
pub fn create_loader() -> Result<SectionLoaderImpl, crate::api::error::config_error::ConfigError> {
    ConfigLoaderFactory::create_loader()
}

/// Create a loader reading from a single explicit directory.
#[inline]
pub fn create_loader_for_dir(dir: impl Into<std::path::PathBuf>) -> SectionLoaderImpl {
    ConfigLoaderFactory::create_loader_for_dir(dir)
}

/// Create a loader following the XDG Base Directory chain for `app_name`.
#[inline]
pub fn create_loader_xdg(
    app_name: &str,
) -> Result<SectionLoaderImpl, crate::api::error::config_error::ConfigError> {
    ConfigLoaderFactory::create_loader_xdg(app_name)
}

/// Create a path validator.
#[inline]
pub fn create_validator() -> PathValidatorImpl {
    ConfigLoaderFactory::create_validator()
}

/// Create a config builder pre-seeded with this package's name and version.
#[inline]
pub fn create_config_builder() -> ConfigBuilderImpl {
    ConfigLoaderFactory::create_config_builder()
}

/// Load the section at `key` as an optional feature, returning `Disabled` when absent.
#[inline]
pub fn load_feature_section<T>(
    loader: &SectionLoaderImpl,
    key: &str,
) -> Result<FeatureState<T>, crate::api::error::config_error::ConfigError>
where
    T: serde::de::DeserializeOwned,
{
    ConfigLoaderFactory::load_feature_section(loader, key)
}

/// Create a loader with environment variable substitution support.
#[inline]
pub fn create_loader_with_substitution(
    policy: Box<dyn SubstitutionPolicy>,
) -> Result<SectionLoaderImpl, crate::api::error::config_error::ConfigError> {
    ConfigLoaderFactory::create_loader_with_substitution(policy)
}

/// Create a loader from a single directory with substitution support.
#[inline]
pub fn create_loader_for_dir_with_substitution(
    dir: impl Into<std::path::PathBuf>,
    policy: Box<dyn SubstitutionPolicy>,
) -> SectionLoaderImpl {
    ConfigLoaderFactory::create_loader_for_dir_with_substitution(dir, policy)
}

/// Create an XDG-aware loader with substitution support.
#[inline]
pub fn create_loader_xdg_with_substitution(
    app_name: &str,
    policy: Box<dyn SubstitutionPolicy>,
) -> Result<SectionLoaderImpl, crate::api::error::config_error::ConfigError> {
    ConfigLoaderFactory::create_loader_xdg_with_substitution(app_name, policy)
}

/// Create a config builder that supports substitution and custom paths.
#[inline]
pub fn create_config_builder_with_substitution(
    policy: Box<dyn SubstitutionPolicy>,
) -> SubstitutionConfigBuilderImpl {
    ConfigLoaderFactory::create_config_builder_with_substitution(policy)
}

#[doc(hidden)]
pub use crate::api::configbuilder::DefaultConfigBuilderBound;
#[doc(hidden)]
pub use crate::api::loader::DefaultSectionLoaderBound;
#[doc(hidden)]
pub use crate::api::substitution::SubstituterBound;
#[doc(hidden)]
pub use crate::api::validator::DefaultValidatorBound;

/// Internal helpers exposed for use by `load_in_order!` and `preflight!` macros.
///
/// Not part of the public API — subject to change without notice.
#[doc(hidden)]
#[allow(missing_docs)]
pub mod __internal {
    pub use crate::api::types::topology::Topology;
}

/// Load a set of optional feature sections in dependency order.
///
/// Computes a topological load order from each type's [`OptionalSection::requires`]
/// declarations and calls [`FeatureRegistry::load`] in that order.  All loaded
/// values are discarded; use `registry.records()` after the call to inspect results.
///
/// Returns `Err(ConfigError::Validation)` if a dependency cycle is detected.
/// The first load failure propagates and stops further loading.
///
/// # Example
///
/// ```rust,ignore
/// use swe_edge_configbuilder::{load_in_order, FeatureRegistry};
///
/// let mut registry = FeatureRegistry::new();
/// load_in_order!(&mut registry, &loader, CacheConfig, BrokerConfig, AnalyticsConfig)?;
/// registry.validate_dependencies()?;
/// println!("{}", registry.summary());
/// ```
#[macro_export]
macro_rules! load_in_order {
    ($registry:expr, $loader:expr, $($ty:ty),+ $(,)?) => {{
        let _names: &[&str] = &[$(<$ty as $crate::OptionalSection>::section_name()),+];
        let _requires: &[&[&str]] = &[$(<$ty as $crate::OptionalSection>::requires()),+];

        match $crate::__internal::Topology::sort(_names, _requires) {
            Err(_msg) => Err($crate::ConfigError::validation("load_in_order", _msg)),
            Ok(_order) => {
                let mut _result: Result<(), $crate::ConfigError> = Ok(());
                'load_loop: for &_idx in &_order {
                    let mut _j: usize = 0;
                    $(
                        if _idx == _j {
                            if let Err(_e) = $registry.load::<$ty>($loader) {
                                _result = Err(_e);
                                break 'load_loop;
                            }
                        }
                        _j += 1;
                    )+
                }
                _result
            }
        }
    }};
}

/// Dry-run feature loading without committing application state.
///
/// Loads every type in topological order into a temporary [`FeatureRegistry`],
/// collecting **all** issues rather than stopping at the first failure.
/// Returns a [`PreflightReport`] the caller can inspect or log before deciding
/// whether to proceed with actual startup.
///
/// Issues reported:
/// - [`PreflightIssueKind::LoadError`] — I/O or parse failure
/// - [`PreflightIssueKind::ValidationError`] — `validate_enabled` rejected the section
/// - [`PreflightIssueKind::DependencyMissing`] — a declared dependency is not enabled
/// - [`PreflightIssueKind::DependencyCycle`] — a cycle exists in the dependency graph
///
/// # Example
///
/// ```rust,ignore
/// use swe_edge_configbuilder::preflight;
///
/// let report = preflight!(&loader, CacheConfig, BrokerConfig, AnalyticsConfig);
/// if !report.is_ok() {
///     eprintln!("{}", report);
///     std::process::exit(1);
/// }
/// ```
#[macro_export]
macro_rules! preflight {
    ($loader:expr, $($ty:ty),+ $(,)?) => {{
        let mut _report = $crate::PreflightReport::new();
        let mut _registry = $crate::FeatureRegistry::new();
        let _names: &[&str] = &[$(<$ty as $crate::OptionalSection>::section_name()),+];
        let _requires: &[&[&str]] = &[$(<$ty as $crate::OptionalSection>::requires()),+];

        match $crate::__internal::Topology::sort(_names, _requires) {
            Err(ref _msg) => {
                _report.push($crate::PreflightIssue {
                    section: String::from("dependency_graph"),
                    kind: $crate::PreflightIssueKind::DependencyCycle,
                    message: _msg.clone(),
                });
            }
            Ok(_order) => {
                for &_idx in &_order {
                    let mut _j: usize = 0;
                    $(
                        if _idx == _j {
                            match _registry.load::<$ty>($loader) {
                                Ok(_) => {}
                                Err(_e) => {
                                    _report.push($crate::PreflightIssue {
                                        section: <$ty as $crate::OptionalSection>::section_name()
                                            .to_owned(),
                                        kind: $crate::PreflightIssueKind::from_config_error(&_e),
                                        message: _e.to_string(),
                                    });
                                }
                            }
                        }
                        _j += 1;
                    )+
                }

                // Capture OnError::Disable degradations as ValidationError issues
                for _record in _registry.records() {
                    if let Some($crate::OverrideSource::ValidationError { ref reason }) =
                        _record.override_source
                    {
                        _report.push($crate::PreflightIssue {
                            section: _record.section_name.clone(),
                            kind: $crate::PreflightIssueKind::ValidationError,
                            message: reason.clone(),
                        });
                    }
                }

                // Check that every enabled feature's dependencies are satisfied
                let _enabled: std::collections::HashSet<&str> = _registry
                    .records()
                    .iter()
                    .filter(|_r| _r.enabled)
                    .map(|_r| _r.section_name.as_str())
                    .collect();

                for _record in _registry.records() {
                    if _record.enabled {
                        for _dep in _record.requires {
                            if !_enabled.contains(_dep) {
                                _report.push($crate::PreflightIssue {
                                    section: _record.section_name.clone(),
                                    kind: $crate::PreflightIssueKind::DependencyMissing,
                                    message: format!(
                                        "requires '{}' but it is not enabled",
                                        _dep
                                    ),
                                });
                            }
                        }
                    }
                }
            }
        }

        _report
    }};
}
