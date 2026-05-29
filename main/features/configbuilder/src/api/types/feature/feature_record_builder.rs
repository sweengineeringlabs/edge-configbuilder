//! [`FeatureRecordBuilder`] — fluent builder for [`FeatureRecord`].

use crate::api::types::feature::feature_metadata::FeatureMetadata;
use crate::api::types::feature::feature_record::FeatureRecord;
use crate::api::types::feature::override_source::OverrideSource;

/// Fluent builder for [`FeatureRecord`].
pub struct FeatureRecordBuilder {
    section_name: String,
    enabled: bool,
    override_source: Option<OverrideSource>,
    requires: &'static [&'static str],
    metadata: FeatureMetadata,
}

impl FeatureRecordBuilder {
    /// Start a builder for the named TOML section.
    pub fn new(section_name: impl Into<String>) -> Self {
        Self {
            section_name: section_name.into(),
            enabled: false,
            override_source: None,
            requires: &[],
            metadata: FeatureMetadata::default(),
        }
    }

    /// Set whether the feature resolved to enabled.
    pub fn enabled(mut self, v: bool) -> Self {
        self.enabled = v;
        self
    }

    /// Set the override source that determined the resolved state.
    pub fn override_source(mut self, v: OverrideSource) -> Self {
        self.override_source = Some(v);
        self
    }

    /// Set the static dependency slice declared by the feature.
    pub fn requires(mut self, v: &'static [&'static str]) -> Self {
        self.requires = v;
        self
    }

    /// Set the static metadata annotations for the feature.
    pub fn metadata(mut self, v: FeatureMetadata) -> Self {
        self.metadata = v;
        self
    }

    /// Consume the builder and return a [`FeatureRecord`].
    pub fn build(self) -> FeatureRecord {
        FeatureRecord {
            section_name: self.section_name,
            enabled: self.enabled,
            override_source: self.override_source,
            requires: self.requires,
            metadata: self.metadata,
        }
    }
}
