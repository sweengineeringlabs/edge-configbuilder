/// Read-only queries on a [`FeatureSummary`] snapshot.
///
/// Implemented by [`FeatureSummary`] in the `core/` layer.
///
/// [`FeatureSummary`]: crate::FeatureSummary
pub trait FeatureSummaryOps {
    /// Count enabled feature records.
    fn enabled_count(&self) -> usize;

    /// Count disabled feature records.
    fn disabled_count(&self) -> usize;

    /// Count total feature records.
    fn total_count(&self) -> usize;

    /// Return `true` when all records are enabled.
    fn all_enabled(&self) -> bool;
}
