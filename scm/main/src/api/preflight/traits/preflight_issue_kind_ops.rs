/// Introspection helpers for [`PreflightIssueKind`] variants.
///
/// Implemented by [`PreflightIssueKind`] in the `core/` layer.
///
/// [`PreflightIssueKind`]: crate::PreflightIssueKind
pub trait PreflightIssueKindOps {
    /// Return the variant's name as a static string.
    fn variant_name(&self) -> &'static str;
}
