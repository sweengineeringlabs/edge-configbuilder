//! [`FeatureSummary`] — point-in-time snapshot of all registered features.

use crate::api::loader::types::feature_record::FeatureRecord;

/// A point-in-time snapshot of every feature loaded through [`FeatureRegistry`].
pub struct FeatureSummary {
    pub(crate) records: Vec<FeatureRecord>,
}
