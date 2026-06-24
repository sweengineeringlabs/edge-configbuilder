use crate::api::loader::types::feature_record::FeatureRecord;

/// Raw output of a feature load before generic deserialisation.
///
/// Used internally by [`crate::api::loader::traits::loader_ops::LoaderOps::load_feature_raw`]
/// to return type-erased feature data that [`crate::api::loader::types::section_loader_impl::SectionLoaderImpl`]
/// then deserialises into the concrete section type.
pub struct RawFeature {
    /// The raw TOML value when the feature is enabled, or `None` when disabled.
    pub(crate) value: Option<toml::Value>,
    /// Metadata record describing the feature's resolved state.
    pub(crate) record: Box<FeatureRecord>,
}
