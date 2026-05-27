//! [`LoadedFeature`] — the full result of loading an optional config section.

use crate::api::types::feature_record::FeatureRecord;
use crate::api::types::feature_state::FeatureState;

/// The full result of loading an optional feature section.
///
/// Returned by [`FeatureLoader::load_feature`] so callers can access both the
/// strongly-typed config value and the metadata record in one call.
/// [`FeatureRegistry`] uses the record to build the startup feature summary.
///
/// [`FeatureLoader::load_feature`]: crate::api::traits::feature_loader::FeatureLoader::load_feature
/// [`FeatureRegistry`]: crate::saf::configbuilder_svc::FeatureRegistry
#[derive(Debug)]
pub struct LoadedFeature<T> {
    /// The feature state after applying all precedence rules:
    /// env var > `enabled = false` TOML flag > section presence > absent.
    pub state: FeatureState<T>,

    /// Metadata describing how the state was determined.
    pub record: FeatureRecord,
}
