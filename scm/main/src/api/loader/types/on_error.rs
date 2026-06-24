//! [`OnError`] — controls how `FeatureRegistry` handles a `validate_enabled` failure.

/// Controls what [`FeatureRegistry`] does when [`OptionalSection::validate_enabled`]
/// returns an error for an otherwise-enabled section.
///
/// Set per feature by overriding [`OptionalSection::on_error`].
/// Override at deploy time via env var `SWE_EDGE_FEATURE_<UPPER_KEY>_ON_ERROR=fail|disable`.
///
/// The env var takes precedence over the trait default.
///
/// [`FeatureRegistry`]: crate::FeatureRegistry
/// [`OptionalSection::validate_enabled`]: crate::OptionalSection::validate_enabled
/// [`OptionalSection::on_error`]: crate::OptionalSection::on_error
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::OnError;
///
/// // Default policy halts startup on misconfiguration.
/// assert_eq!(OnError::default(), OnError::Fail);
///
/// // Override to keep the service running even when this feature is broken.
/// let policy = OnError::Disable;
/// assert_ne!(policy, OnError::Fail);
/// ```
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
    /// [`FeatureRecord::override_source`]: crate::api::loader::types::feature_record::FeatureRecord::override_source
    /// [`OverrideSource::ValidationError`]: crate::api::loader::types::override_source::OverrideSource::ValidationError
    Disable,
}
