//! [`FeatureState`] — presence-driven feature gate for optional config sections.

/// Whether an optional TOML section is present and active.
///
/// `load_optional_section` returns `Enabled(T)` when the key exists in any
/// config file and `Disabled` when it is absent from all of them.  Absent
/// means the feature is intentionally off — no error is raised.
///
/// Use [`FeatureRegistry::load`] for full startup coordination (graceful
/// degradation, dependency validation, observer hooks). Use
/// [`ConfigLoaderFactory::load_feature_section`] for one-off ad-hoc loading.
///
/// [`FeatureRegistry::load`]: crate::FeatureRegistry::load
/// [`ConfigLoaderFactory::load_feature_section`]: crate::ConfigLoaderFactory::load_feature_section
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::FeatureState;
///
/// let enabled: FeatureState<u32> = FeatureState::Enabled(42);
/// assert!(enabled.is_enabled());
/// assert_eq!(enabled.into_option(), Some(42));
///
/// let disabled: FeatureState<u32> = FeatureState::Disabled;
/// assert!(disabled.is_disabled());
/// assert_eq!(disabled.into_option(), None);
/// ```
pub enum FeatureState<T> {
    /// The section key is present in TOML and deserialized successfully into `T`.
    Enabled(T),
    /// The section key is absent from every config file — the feature is off.
    Disabled,
}
