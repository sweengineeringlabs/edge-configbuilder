//! [`OverrideSource`] — why a feature's state was forced by an external control.

/// Records which external control overrode a feature's natural TOML state.
///
/// When [`FeatureRecord::override_source`] is `Some`, the feature's enabled/disabled
/// state was forced by one of these mechanisms rather than simple section presence.
///
/// [`FeatureRecord::override_source`]: crate::api::types::feature_record::FeatureRecord::override_source
#[derive(Debug, Clone)]
pub enum OverrideSource {
    /// An environment variable forced this feature on or off.
    ///
    /// Takes precedence over both TOML section presence and the `enabled` field.
    EnvVar {
        /// The full environment variable name (e.g. `SWE_EDGE_FEATURE_MESSAGE_BROKER`).
        var_name: String,
        /// The raw value that was set (e.g. `"false"`, `"true"`, `"0"`).
        value: String,
    },
    /// The TOML section is present but `enabled = false` was set explicitly.
    ///
    /// This lets operators keep config values in place while temporarily
    /// disabling the feature — useful for staged rollouts and quick kill-switches.
    ExplicitTomlFlag,
}
