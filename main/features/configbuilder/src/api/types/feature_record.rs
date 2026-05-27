//! [`FeatureRecord`] — a snapshot of one feature's state and why it got there.

use crate::api::types::override_source::OverrideSource;

/// A snapshot of one optional feature's resolved state.
///
/// Collected by [`FeatureRegistry`] as each feature is loaded, so the registry
/// can produce a startup summary of every registered feature.
///
/// [`FeatureRegistry`]: crate::saf::configbuilder_svc::FeatureRegistry
#[derive(Debug, Clone)]
pub struct FeatureRecord {
    /// The TOML section key for this feature (e.g. `"message_broker"`).
    pub section_name: String,

    /// Whether the feature resolved to enabled after applying all precedence rules.
    pub enabled: bool,

    /// Which external control overrode the natural TOML state, if any.
    ///
    /// `None` means the state came from normal TOML logic (section presence or
    /// absence); `Some` means an env var or explicit `enabled = false` flag took
    /// precedence.
    pub override_source: Option<OverrideSource>,
}
