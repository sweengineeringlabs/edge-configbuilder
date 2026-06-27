use crate::api::{
    ConfigError, FeatureRecord, FeatureState, FeatureSummary, OptionalSection, SectionLoaderImpl,
};

/// Startup feature registry: collection, dependency validation, and observation.
///
/// Implemented by [`FeatureRegistry`] in the `core/` layer.
///
/// [`FeatureRegistry`]: crate::FeatureRegistry
pub trait FeatureRegistryOps: Sized {
    /// Create an empty registry.
    fn new() -> Self;

    /// Register a callback to observe loaded records.
    fn on_load(&mut self, observer: impl Fn(&FeatureRecord) + 'static);

    /// Load a feature section and record its state.
    ///
    /// # Errors
    ///
    /// Propagates any [`ConfigError`] returned by the loader.
    fn load<T>(&mut self, loader: &SectionLoaderImpl) -> Result<FeatureState<T>, ConfigError>
    where
        T: OptionalSection;

    /// Validate that every enabled feature's declared dependencies are enabled.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Validation`] listing all unsatisfied dependencies.
    fn validate_dependencies(&self) -> Result<(), ConfigError>;

    /// Borrow the recorded feature records.
    fn records(&self) -> &[FeatureRecord];

    /// Build a point-in-time snapshot of the recorded features.
    fn summary(&self) -> FeatureSummary;
}
