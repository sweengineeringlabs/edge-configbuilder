//! [`FeatureRegistry`] — startup feature collector and dependency validator.

use crate::api::loader::traits::optional_section::OptionalSection;
use crate::api::loader::types::feature_record::FeatureRecord;
use crate::api::loader::types::feature_state::FeatureState;
use crate::api::loader::types::feature_summary::FeatureSummary;
use crate::api::loader::types::on_error::OnError;
use crate::api::loader::types::override_source::OverrideSource;
use crate::api::loader::types::section_loader_impl::SectionLoaderImpl;
use crate::api::ConfigError;

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
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_configbuilder::{ConfigLoaderFactory, FeatureRegistry, OptionalSection};
///
/// # #[derive(serde::Deserialize)] struct BrokerConfig { host: String }
/// # impl OptionalSection for BrokerConfig { fn section_name() -> &'static str { "broker" } }
/// # #[derive(serde::Deserialize)] struct CacheConfig { ttl: u64 }
/// # impl OptionalSection for CacheConfig { fn section_name() -> &'static str { "cache" } }
/// let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
///
/// let mut registry = FeatureRegistry::new();
/// registry.on_load(|r| eprintln!("[{}] enabled={}", r.section_name, r.enabled));
///
/// let _broker = registry.load::<BrokerConfig>(&loader).expect("load failed");
/// let _cache  = registry.load::<CacheConfig>(&loader).expect("load failed");
///
/// registry.validate_dependencies().expect("dependency check failed");
/// eprintln!("{}", registry.summary());
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
    /// Create an empty registry with no features and no observers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureRegistry;
    /// let registry = FeatureRegistry::new();
    /// assert_eq!(registry.records().len(), 0);
    /// assert!(registry.summary().all_enabled()); // vacuously true
    /// ```
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
    pub fn load<T>(&mut self, loader: &SectionLoaderImpl) -> Result<FeatureState<T>, ConfigError>
    where
        T: OptionalSection,
    {
        use crate::api::loader::types::loaded_feature::LoadedFeature;

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
            Some(Err(e)) => match Self::resolve_on_error::<T>(T::section_name()) {
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
            section_name: record.section_name,
            enabled: final_state.is_enabled(),
            override_source: final_override,
            requires: T::requires(),
            metadata: Box::new(T::metadata()),
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureRegistry;
    /// let registry = FeatureRegistry::new();
    /// assert!(registry.records().is_empty());
    /// ```
    pub fn records(&self) -> &[FeatureRecord] {
        &self.records
    }

    /// Produce a startup summary of every registered feature.
    ///
    /// The returned [`FeatureSummary`] implements [`Display`] — log it directly
    /// with `tracing::info!("{}", registry.summary())`.
    ///
    /// [`FeatureSummary`]: crate::FeatureSummary
    /// [`Display`]: std::fmt::Display
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureRegistry;
    /// let registry = FeatureRegistry::new();
    /// let summary = registry.summary();
    /// assert_eq!(summary.total_count(), 0);
    /// ```
    pub fn summary(&self) -> FeatureSummary {
        FeatureSummary {
            records: self.records.clone(),
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
}
