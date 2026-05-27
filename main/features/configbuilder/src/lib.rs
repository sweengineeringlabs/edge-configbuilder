//! Standalone, runtime-independent TOML section loader for swe-edge services.
//!
//! Provides XDG-aware, layered config section loading for any
//! `T: DeserializeOwned + Default`. Library crates can depend on this crate
//! directly without pulling in `swe-edge-runtime-main`.
//!
//! # Usage
//!
//! ```rust,ignore
//! use swe_edge_configbuilder::{create_loader, Loader};
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
mod saf;
pub mod spi;

pub use crate::api::feature::traits::feature_loader::FeatureLoader;
pub use crate::api::loader::traits::loader::Loader;
pub use crate::api::loader::traits::substitution_policy::SubstitutionPolicy;
pub use crate::api::types::feature::{
    FeatureMetadata, FeatureRecord, FeatureState, LoadedFeature, OnError, OverrideSource,
};
pub use crate::api::types::loader::{
    AllowAllPolicy, CompositePolicy, PatternWhitelistPolicy, PrefixWhitelistPolicy,
};
pub use crate::api::types::preflight::{PreflightIssue, PreflightIssueKind, PreflightReport};
pub use saf::*;
pub use spi::{ConfigSection, OptionalSection};

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
///   (both hard `Fail` and graceful `Disable` degradations are captured)
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
