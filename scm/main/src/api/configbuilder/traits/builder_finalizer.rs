/// Consume a configured builder and produce a ready-to-use section loader.
///
/// Implemented by `ConfigBuilderImpl` and `SubstitutionConfigBuilderImpl`
/// in the `core/` layer so construction logic can access internal types.
pub trait BuilderFinalizer {
    /// The concrete loader type produced by this builder.
    type Loader;

    /// The error type returned on build failure.
    type Error: std::error::Error;

    /// Consume the builder and return a ready-to-use section loader.
    ///
    /// # Errors
    ///
    /// Returns an error if the resolved config directory path contains `..`
    /// traversal components, or if a resolved path exists but is not a directory.
    fn build_loader(self) -> Result<Self::Loader, Self::Error>;
}
