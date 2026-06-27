//! Standalone, runtime-independent TOML section loader for swe-edge services.
//!
//! Provides XDG-aware, layered config section loading for any
//! `T: DeserializeOwned + Default`. Library crates can depend on this crate
//! directly without pulling in `swe-edge-runtime-main`.
//!
//! # Usage
//!
//! ```rust,no_run
//! use swe_edge_configbuilder::ConfigLoaderFactory;
//!
//! #[derive(serde::Deserialize, Default)]
//! struct CompletionConfig { model: String, max_tokens: u32 }
//!
//! let cfg: CompletionConfig = ConfigLoaderFactory::create_loader()
//!     .expect("config dir accessible")
//!     .load_section("application.completion")
//!     .expect("completion section required");
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;

#[cfg(any(test, feature = "test-utils"))]
pub use crate::api::AllowAllPolicy;
pub use crate::api::ApplicationConfig;
pub use crate::api::ConfigBuilder;
pub use crate::api::ConfigError;
pub use crate::api::ConfigSection;
pub use crate::api::FeatureLoader;
pub use crate::api::FeatureRegistry;
pub use crate::api::FeatureSummary;
pub use crate::api::Loader;
pub use crate::api::OptionalSection;
pub use crate::api::Preflight;
pub use crate::api::SubstitutionError;
pub use crate::api::SubstitutionPolicy;
pub use crate::api::Validator;
pub use crate::api::ValidatorError;
pub use crate::api::{CompositePolicy, PatternWhitelistPolicy, PrefixWhitelistPolicy};
pub use crate::api::{
    FeatureMetadata, FeatureRecord, FeatureRecordBuilder, FeatureState, LoadedFeature, OnError,
    OverrideSource,
};
pub use crate::api::{PreflightIssue, PreflightIssueKind, PreflightReport};
pub use saf::*;

#[doc(hidden)]
pub use crate::api::ConfigBuilderBound;
#[doc(hidden)]
pub use crate::api::PolicyCatalog;
#[doc(hidden)]
pub use crate::api::SectionLoaderBound;
#[doc(hidden)]
pub use crate::api::SubstituterBound;
#[doc(hidden)]
pub use crate::api::ValidatorBound;

#[doc(hidden)]
pub use crate::api::Topology;

/// Load a set of optional feature sections in dependency order.
///
/// Computes a topological load order from each type's [`OptionalSection::requires`]
/// declarations and calls [`FeatureRegistry::load`] in that order.  All loaded
/// values are discarded; use `registry.records()` after the call to inspect results.
///
/// Returns `Err(ConfigError::Validation)` if a dependency cycle is detected.
/// The first load failure propagates and stops further loading.
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_configbuilder::{load_in_order, ConfigLoaderFactory, OptionalSection};
///
/// # #[derive(serde::Deserialize)] struct CacheConfig;
/// # impl OptionalSection for CacheConfig { fn section_name() -> &'static str { "cache" } }
/// # #[derive(serde::Deserialize)] struct BrokerConfig;
/// # impl OptionalSection for BrokerConfig { fn section_name() -> &'static str { "broker" } }
/// let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
/// let mut registry = ConfigLoaderFactory::create_feature_registry();
///
/// load_in_order!(&mut registry, &loader, CacheConfig, BrokerConfig)
///     .expect("dependency order must be acyclic");
///
/// ConfigLoaderFactory::feature_registry_validate_dependencies(&registry)
///     .expect("all dependencies satisfied");
/// println!("{}", ConfigLoaderFactory::feature_registry_summary(&registry));
/// ```
#[macro_export]
macro_rules! load_in_order {
    ($registry:expr, $loader:expr, $($ty:ty),+ $(,)?) => {{
        let _names: &[&str] = &[$(<$ty as $crate::OptionalSection>::section_name()),+];
        let _requires: &[&[&str]] = &[$(<$ty as $crate::OptionalSection>::requires()),+];

        match $crate::ConfigLoaderFactory::topology_sort(_names, _requires) {
            Err(_msg) => Err($crate::ConfigError::Validation {
                section: String::from("load_in_order"),
                reason: _msg,
            }),
            Ok(_order) => {
                let mut _result: Result<(), $crate::ConfigError> = Ok(());
                'load_loop: for &_idx in &_order {
                    let mut _j: usize = 0;
                    $(
                        if _idx == _j {
                            if let Err(_e) = $crate::ConfigLoaderFactory::feature_registry_load::<$ty>($registry, $loader) {
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
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_configbuilder::{preflight, ConfigLoaderFactory, OptionalSection};
///
/// # #[derive(serde::Deserialize)] struct CacheConfig;
/// # impl OptionalSection for CacheConfig { fn section_name() -> &'static str { "cache" } }
/// # #[derive(serde::Deserialize)] struct BrokerConfig;
/// # impl OptionalSection for BrokerConfig { fn section_name() -> &'static str { "broker" } }
/// let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
/// let report = preflight!(&loader, CacheConfig, BrokerConfig);
///
/// if !report.is_ok() {
///     eprintln!("{report}");
///     std::process::exit(1);
/// }
/// ```
#[macro_export]
macro_rules! preflight {
    ($loader:expr, $($ty:ty),+ $(,)?) => {{
        let mut _report = $crate::ConfigLoaderFactory::create_preflight_report();
        let mut _registry = $crate::ConfigLoaderFactory::create_feature_registry();
        let _names: &[&str] = &[$(<$ty as $crate::OptionalSection>::section_name()),+];
        let _requires: &[&[&str]] = &[$(<$ty as $crate::OptionalSection>::requires()),+];

        match $crate::ConfigLoaderFactory::topology_sort(_names, _requires) {
            Err(ref _msg) => {
                $crate::ConfigLoaderFactory::preflight_report_push(
                    &mut _report,
                    $crate::PreflightIssue {
                        section: String::from("dependency_graph"),
                        kind: $crate::PreflightIssueKind::DependencyCycle,
                        message: _msg.clone(),
                    },
                );
            }
            Ok(_order) => {
                for &_idx in &_order {
                    let mut _j: usize = 0;
                    $(
                        if _idx == _j {
                            match $crate::ConfigLoaderFactory::feature_registry_load::<$ty>(&mut _registry, $loader) {
                                Ok(_) => {}
                                Err(_e) => {
                                    $crate::ConfigLoaderFactory::preflight_report_push(
                                        &mut _report,
                                        $crate::PreflightIssue {
                                            section: <$ty as $crate::OptionalSection>::section_name()
                                                .to_owned(),
                                            kind: $crate::PreflightIssueKind::from_config_error(&_e),
                                            message: _e.to_string(),
                                        },
                                    );
                                }
                            }
                        }
                        _j += 1;
                    )+
                }

                // Capture OnError::Disable degradations as ValidationError issues
                for _record in $crate::ConfigLoaderFactory::feature_registry_records(&_registry) {
                    if let Some($crate::OverrideSource::ValidationError { ref reason }) =
                        _record.override_source
                    {
                        $crate::ConfigLoaderFactory::preflight_report_push(
                            &mut _report,
                            $crate::PreflightIssue {
                                section: _record.section_name.clone(),
                                kind: $crate::PreflightIssueKind::ValidationError,
                                message: reason.clone(),
                            },
                        );
                    }
                }

                // Check that every enabled feature's dependencies are satisfied
                let _enabled: std::collections::HashSet<&str> = $crate::ConfigLoaderFactory::feature_registry_records(&_registry)
                    .iter()
                    .filter(|_r| _r.enabled)
                    .map(|_r| _r.section_name.as_str())
                    .collect();

                for _record in $crate::ConfigLoaderFactory::feature_registry_records(&_registry) {
                    if _record.enabled {
                        for _dep in _record.requires {
                            if !_enabled.contains(_dep) {
                                $crate::ConfigLoaderFactory::preflight_report_push(
                                    &mut _report,
                                    $crate::PreflightIssue {
                                        section: _record.section_name.clone(),
                                        kind: $crate::PreflightIssueKind::DependencyMissing,
                                        message: format!(
                                            "requires '{}' but it is not enabled",
                                            _dep
                                        ),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }

        _report
    }};
}
