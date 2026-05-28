//! [`OnError`] — controls how `FeatureRegistry` handles a `validate_enabled` failure.

/// Controls what [`FeatureRegistry`] does when [`OptionalSection::validate_enabled`]
/// returns an error for an otherwise-enabled section.
///
/// Set per feature by overriding [`OptionalSection::on_error`].
/// Override at deploy time via env var `SWE_EDGE_FEATURE_<UPPER_KEY>_ON_ERROR=fail|disable`.
///
/// The env var takes precedence over the trait default.
///
/// [`FeatureRegistry`]: crate::api::types::feature_registry::FeatureRegistry
/// [`OptionalSection::validate_enabled`]: crate::api::traits::optional_section::OptionalSection::validate_enabled
/// [`OptionalSection::on_error`]: crate::api::traits::optional_section::OptionalSection::on_error
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OnError {
    /// Propagate the error — startup halts immediately. **This is the default.**
    ///
    /// Use this for mandatory features where a misconfiguration must not be silently
    /// ignored (e.g. auth, TLS, database connections).
    #[default]
    Fail,

    /// Treat the feature as disabled and continue startup.
    ///
    /// The validation error reason is recorded in [`FeatureRecord::override_source`]
    /// as [`OverrideSource::ValidationError`] so operators can inspect it in the
    /// startup summary without crashing the service.
    ///
    /// Use this for optional/enhancement features (analytics, tracing exporters,
    /// feature flags) where a config mistake should not block the critical path.
    ///
    /// [`FeatureRecord::override_source`]: crate::api::types::feature::feature_record::FeatureRecord::override_source
    /// [`OverrideSource::ValidationError`]: crate::api::types::feature::override_source::OverrideSource::ValidationError
    Disable,
}
