//! [`FeatureSummary`] — point-in-time snapshot of all registered features.

use std::fmt;

use crate::{FeatureSummary, OverrideSource};

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

impl crate::api::FeatureSummaryOps for FeatureSummary {
    fn enabled_count(&self) -> usize {
        FeatureSummary::enabled_count(self)
    }

    fn disabled_count(&self) -> usize {
        FeatureSummary::disabled_count(self)
    }

    fn total_count(&self) -> usize {
        FeatureSummary::total_count(self)
    }

    fn all_enabled(&self) -> bool {
        FeatureSummary::all_enabled(self)
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
