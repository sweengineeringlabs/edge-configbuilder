use crate::api::{FeatureMetadata, FeatureRecord, OverrideSource};

/// Fluent builder operations for constructing [`FeatureRecord`] instances.
///
/// Implemented by [`FeatureRecordBuilder`] in the `core/` layer.
///
/// [`FeatureRecord`]: crate::FeatureRecord
/// [`FeatureRecordBuilder`]: crate::FeatureRecordBuilder
pub trait FeatureRecordBuilderOps: Sized {
    /// Create a new record builder for the named section.
    fn new(section_name: impl Into<String>) -> Self;

    /// Mark the feature as enabled or disabled.
    fn enabled(self, v: bool) -> Self;

    /// Record the source that overrode the feature state.
    fn override_source(self, v: OverrideSource) -> Self;

    /// Attach the required feature dependencies.
    fn requires(self, v: &'static [&'static str]) -> Self;

    /// Attach feature metadata to the record under construction.
    fn metadata(self, v: FeatureMetadata) -> Self;

    /// Finalise the builder and return the feature record.
    fn build(self) -> FeatureRecord;
}
