use crate::api::ConfigError;

/// Topological dependency resolver for feature load ordering.
///
/// Implemented by [`Topology`] in the `core/` layer.
///
/// [`Topology`]: crate::Topology
pub trait TopologyOps {
    /// Return a topological ordering of the provided names.
    ///
    /// `names` is the list of feature names; `requires[i]` is the slice of
    /// dependencies for `names[i]`.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Validation`] listing all nodes involved in a detected cycle.
    fn sort(&self, names: &[&str], requires: &[&[&str]]) -> Result<Vec<usize>, ConfigError>;
}
