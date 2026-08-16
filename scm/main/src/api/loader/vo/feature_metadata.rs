//! [`FeatureMetadata`] — static annotations for an optional feature section.

/// Static documentation and ownership annotations for an optional feature.
///
/// Returned by [`OptionalSection::metadata`] and embedded in [`FeatureRecord`]
/// so the startup [`FeatureSummary`] can produce self-documenting operator output.
///
/// All fields are `&'static str` so the struct is `Copy` and zero-cost at runtime.
/// Use struct-literal syntax with `..FeatureMetadata::default()` to set only the
/// fields you care about.
///
/// [`OptionalSection::metadata`]: crate::OptionalSection::metadata
/// [`FeatureRecord`]: crate::FeatureRecord
/// [`FeatureSummary`]: crate::FeatureSummary
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::FeatureMetadata;
///
/// let meta = FeatureMetadata {
///     description: "Async message bus backed by NATS",
///     owner: "platform-team",
///     deprecated_since: None,
/// };
///
/// assert_eq!(meta.owner, "platform-team");
/// assert!(meta.deprecated_since.is_none());
///
/// // Partial construction via Default
/// let minimal = FeatureMetadata { description: "TLS termination", ..Default::default() };
/// assert!(minimal.owner.is_empty());
/// ```
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
