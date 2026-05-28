//! [`FeatureSummary`] — point-in-time snapshot of all registered features.

use std::fmt;

use crate::api::types::feature::feature_record::FeatureRecord;
use crate::api::types::feature::override_source::OverrideSource;

/// A point-in-time snapshot of every feature loaded through [`FeatureRegistry`].
///
/// Implements [`Display`] so you can log it directly: `tracing::info!("{}", summary)`.
///
/// [`FeatureRegistry`]: crate::api::types::feature_registry::FeatureRegistry
/// [`Display`]: std::fmt::Display
pub struct FeatureSummary {
    pub(crate) records: Vec<FeatureRecord>,
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
