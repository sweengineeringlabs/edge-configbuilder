//! [`FeatureSummary`] — point-in-time snapshot of all registered features.

use crate::api::loader::types::feature_record::FeatureRecord;

/// A point-in-time snapshot of every feature loaded through the registry facade.
pub struct FeatureSummary {
    pub(crate) records: Vec<FeatureRecord>,
}
