//! [`FeatureSummary`] — point-in-time snapshot of all registered features.

use std::fmt;

use crate::{FeatureSummary, OverrideSource};

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
