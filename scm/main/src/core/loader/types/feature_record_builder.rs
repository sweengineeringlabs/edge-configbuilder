//! [`FeatureRecordBuilder`] — fluent builder for [`FeatureRecord`].

use crate::api::{
    FeatureMetadata, FeatureRecord, FeatureRecordBuilder, FeatureRecordBuilderOps, OverrideSource,
};

impl FeatureRecordBuilderOps for FeatureRecordBuilder {
    fn new(section_name: impl Into<String>) -> Self {
        Self {
            section_name: section_name.into(),
            enabled: false,
            override_source: None,
            requires: &[],
            metadata: Box::new(FeatureMetadata::default()),
        }
    }

    fn enabled(mut self, v: bool) -> Self {
        self.enabled = v;
        self
    }

    fn override_source(mut self, v: OverrideSource) -> Self {
        self.override_source = Some(v);
        self
    }

    fn requires(mut self, v: &'static [&'static str]) -> Self {
        self.requires = v;
        self
    }

    fn metadata(mut self, v: FeatureMetadata) -> Self {
        self.metadata = Box::new(v);
        self
    }

    fn build(self) -> FeatureRecord {
        FeatureRecord {
            section_name: self.section_name,
            enabled: self.enabled,
            override_source: self.override_source,
            requires: self.requires,
            metadata: self.metadata,
        }
    }
}
