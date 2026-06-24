//! [`LoadedFeature`] — the full result of loading an optional config section.

use crate::api::loader::types::feature_record::FeatureRecord;
use crate::api::loader::types::feature_state::FeatureState;

/// The full result of loading an optional feature section.
///
/// Returned by [`FeatureLoader::load_feature`] so callers can access both the
/// strongly-typed config value and the metadata record in one call.
/// [`FeatureRegistry`] uses the record to build the startup feature summary.
///
/// You do not construct this directly — it is returned by the loader.
/// Use [`state`] to check enabled/disabled and [`record`] to inspect why.
///
/// [`FeatureLoader::load_feature`]: crate::FeatureLoader::load_feature
/// [`FeatureRegistry`]: crate::FeatureRegistry
/// [`state`]: LoadedFeature::state
/// [`record`]: LoadedFeature::record
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::{FeatureRecordBuilder, FeatureState, LoadedFeature};
///
/// // Simulating what the loader returns for an enabled section.
/// let loaded = LoadedFeature {
///     state: FeatureState::Enabled(42_u32),
///     record: Box::new(FeatureRecordBuilder::new("counter").enabled(true).build()),
/// };
///
/// assert!(loaded.state.is_enabled());
/// assert_eq!(loaded.record.section_name, "counter");
/// ```
#[derive(Debug)]
pub struct LoadedFeature<T> {
    /// The feature state after applying all precedence rules:
    /// env var > `enabled = false` TOML flag > section presence > absent.
    pub state: FeatureState<T>,

    /// Metadata describing how the state was determined.
    pub record: Box<FeatureRecord>,
}
