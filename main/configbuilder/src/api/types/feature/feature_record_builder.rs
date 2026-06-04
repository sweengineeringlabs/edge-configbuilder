//! [`FeatureRecordBuilder`] — fluent builder for [`FeatureRecord`].

use crate::api::types::feature::feature_metadata::FeatureMetadata;
use crate::api::types::feature::feature_record::FeatureRecord;
use crate::api::types::feature::override_source::OverrideSource;

/// Fluent builder for [`FeatureRecord`].
///
/// Start with [`FeatureRecordBuilder::new`], chain the setter methods, then call
/// [`build`] to obtain a [`FeatureRecord`].  Fields not explicitly set default
/// to: `enabled = false`, no override source, no dependencies, empty metadata.
///
/// [`build`]: FeatureRecordBuilder::build
/// [`FeatureRecord`]: crate::FeatureRecord
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::{FeatureMetadata, FeatureRecordBuilder, OverrideSource};
///
/// let record = FeatureRecordBuilder::new("message_broker")
///     .enabled(true)
///     .requires(&["tls"])
///     .metadata(FeatureMetadata {
///         description: "Async message bus",
///         owner: "platform-team",
///         deprecated_since: None,
///     })
///     .build();
///
/// assert_eq!(record.section_name, "message_broker");
/// assert!(record.enabled);
/// assert_eq!(record.metadata.description, "Async message bus");
/// ```
pub struct FeatureRecordBuilder {
    section_name: String,
    enabled: bool,
    override_source: Option<OverrideSource>,
    requires: &'static [&'static str],
    metadata: FeatureMetadata,
}

impl FeatureRecordBuilder {
    /// Start a builder for the named TOML section.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureRecordBuilder;
    /// let b = FeatureRecordBuilder::new("auth");
    /// assert_eq!(b.build().section_name, "auth");
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::FeatureRecordBuilder;
    /// let record = FeatureRecordBuilder::new("tls").enabled(true).build();
    /// assert!(record.enabled);
    /// assert!(record.override_source.is_none());
    /// ```
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
