//! [`FeatureRecord`] — a snapshot of one feature's state and why it got there.

use crate::api::loader::types::feature_metadata::FeatureMetadata;
use crate::api::loader::types::override_source::OverrideSource;

/// A snapshot of one optional feature's resolved state.
///
/// Collected by [`FeatureRegistry`] as each feature is loaded, so the registry
/// can produce a startup summary of every registered feature.
///
/// Construct via [`FeatureRecordBuilder`] rather than struct literal syntax, as
/// new fields may be added without a semver bump.
///
/// [`FeatureRegistry`]: crate::api::loader::types::feature_registry::FeatureRegistry
/// [`FeatureRecordBuilder`]: crate::FeatureRecordBuilder
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::FeatureRecordBuilder;
///
/// let record = FeatureRecordBuilder::new("cache")
///     .enabled(true)
///     .requires(&["tls"])
///     .build();
///
/// assert_eq!(record.section_name, "cache");
/// assert!(record.enabled);
/// assert_eq!(record.requires, &["tls"]);
/// assert!(record.override_source.is_none());
/// ```
#[derive(Debug, Clone)]
pub struct FeatureRecord {
    /// The TOML section key for this feature (e.g. `"message_broker"`).
    pub section_name: String,

    /// Whether the feature resolved to enabled after applying all precedence rules.
    pub enabled: bool,

    /// Which external control overrode the natural TOML state, if any.
    ///
    /// `None` means the state came from normal TOML logic (section presence or
    /// absence); `Some` means an env var, explicit `enabled = false` flag, or
    /// graceful-degradation policy took precedence.
    pub override_source: Option<OverrideSource>,

    /// Section keys this feature declared it depends on via
    /// [`OptionalSection::requires`].
    ///
    /// [`OptionalSection::requires`]: crate::api::loader::traits::optional_section::OptionalSection::requires
    pub requires: &'static [&'static str],

    /// Static annotations (description, owner, deprecation) declared by the
    /// feature via [`OptionalSection::metadata`].
    ///
    /// [`OptionalSection::metadata`]: crate::api::loader::traits::optional_section::OptionalSection::metadata
    pub metadata: Box<FeatureMetadata>,
}
