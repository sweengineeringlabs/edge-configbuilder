//! [`FeatureSummary`] — point-in-time snapshot of all registered features.

use std::fmt;

use crate::api::loader::types::feature_record::FeatureRecord;
use crate::api::loader::types::override_source::OverrideSource;

/// A point-in-time snapshot of every feature loaded through [`FeatureRegistry`].
///
/// Implements [`Display`] so you can log it directly: `tracing::info!("{}", summary)`.
/// Each line shows `[ON]` or `[OFF]` followed by the section name and, when present,
/// an override note (env var, explicit flag, or graceful-degradation reason),
/// description, owner, and deprecation notice.
///
/// Obtain via [`FeatureRegistry::summary`], not by construction.
///
/// [`FeatureRegistry`]: crate::FeatureRegistry
/// [`FeatureRegistry::summary`]: crate::FeatureRegistry::summary
/// [`Display`]: std::fmt::Display
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::FeatureRegistry;
///
/// let registry = FeatureRegistry::new();
/// let summary = registry.summary();
///
/// assert_eq!(summary.total_count(), 0);
/// assert_eq!(summary.enabled_count(), 0);
/// assert!(summary.all_enabled()); // vacuously true for an empty registry
///
/// // Useful for logging:
/// let text = summary.to_string();
/// assert!(text.starts_with("features:"));
/// ```
pub struct FeatureSummary {
    pub(crate) records: Vec<FeatureRecord>,
}

impl FeatureSummary {
    /// Number of features that resolved to enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureRegistry;
    /// assert_eq!(FeatureRegistry::new().summary().enabled_count(), 0);
    /// ```
    pub fn enabled_count(&self) -> usize {
        self.records.iter().filter(|r| r.enabled).count()
    }

    /// Number of features that resolved to disabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureRegistry;
    /// assert_eq!(FeatureRegistry::new().summary().disabled_count(), 0);
    /// ```
    pub fn disabled_count(&self) -> usize {
        self.records.iter().filter(|r| !r.enabled).count()
    }

    /// Total number of registered features.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureRegistry;
    /// assert_eq!(FeatureRegistry::new().summary().total_count(), 0);
    /// ```
    pub fn total_count(&self) -> usize {
        self.records.len()
    }

    /// Returns `true` when every registered feature resolved to enabled.
    ///
    /// Returns `true` for an empty registry (vacuously true).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureRegistry;
    /// // Empty registry: all_enabled is vacuously true.
    /// assert!(FeatureRegistry::new().summary().all_enabled());
    /// ```
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
