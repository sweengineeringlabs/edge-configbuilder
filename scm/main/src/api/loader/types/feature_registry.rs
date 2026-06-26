//! [`FeatureRegistry`] — startup feature collector and dependency validator.

use crate::api::loader::types::feature_record::FeatureRecord;

type FeatureObserver = Box<dyn Fn(&FeatureRecord)>;

/// Collects feature-load metadata at startup for all optional TOML sections.
pub struct FeatureRegistry {
    pub(crate) records: Vec<FeatureRecord>,
    pub(crate) observers: Vec<FeatureObserver>,
}
