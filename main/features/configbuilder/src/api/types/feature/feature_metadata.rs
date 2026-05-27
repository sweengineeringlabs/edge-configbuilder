//! [`FeatureMetadata`] — static annotations for an optional feature section.

/// Static documentation and ownership annotations for an optional feature.
///
/// Returned by [`OptionalSection::metadata`] and embedded in [`FeatureRecord`]
/// so the startup [`FeatureSummary`] can produce self-documenting operator output.
///
/// All fields are `&'static str` so the struct is `Copy` and zero-cost at runtime.
///
/// [`OptionalSection::metadata`]: crate::spi::OptionalSection::metadata
/// [`FeatureRecord`]: crate::api::types::feature::feature_record::FeatureRecord
/// [`FeatureSummary`]: crate::saf::configbuilder_svc::FeatureSummary
#[derive(Debug, Clone, Copy, Default)]
pub struct FeatureMetadata {
    /// Short human-readable description shown in startup summaries and operator logs.
    pub description: &'static str,

    /// Team or individual responsible for this feature (e.g. `"platform-team"`).
    pub owner: &'static str,

    /// If set, marks this feature as deprecated as of the given version string
    /// (e.g. `Some("0.4.0")`).  Shown as `[DEPRECATED since 0.4.0]` in the summary.
    pub deprecated_since: Option<&'static str>,
}
