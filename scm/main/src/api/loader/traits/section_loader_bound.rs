//! Interface counterpart for [`crate::core::loader::default_section_loader::DefaultSectionLoader`].

use crate::api::{
    FeatureRecord, FeatureRecordBuilder, FeatureRegistry, FeatureSummary, OverrideSource, Topology,
};

/// API contract marker for the default section loader.
///
/// The concrete implementor is
/// `crate::core::loader::default_section_loader::DefaultSectionLoader`.
pub trait SectionLoaderBound {
    /// Feature outcome record type produced by feature loading.
    type FeatureRecord: Into<FeatureRecord>;

    /// Builder for feature outcome records.
    type FeatureRecordBuilder: Into<FeatureRecordBuilder>;

    /// Registry collecting feature outcome records.
    type FeatureRegistry: Into<FeatureRegistry>;

    /// Summary view produced by the registry.
    type FeatureSummary: Into<FeatureSummary>;

    /// Source of a feature state override.
    type OverrideSource: Into<OverrideSource>;

    /// Dependency ordering resolver used by feature macros.
    type Topology: Into<Topology>;
}
