//! [`FeatureRecordBuilder`] — fluent builder for [`FeatureRecord`].

use crate::api::loader::types::feature_metadata::FeatureMetadata;
use crate::api::loader::types::override_source::OverrideSource;

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
    pub(crate) section_name: String,
    pub(crate) enabled: bool,
    pub(crate) override_source: Option<OverrideSource>,
    pub(crate) requires: &'static [&'static str],
    pub(crate) metadata: Box<FeatureMetadata>,
}
