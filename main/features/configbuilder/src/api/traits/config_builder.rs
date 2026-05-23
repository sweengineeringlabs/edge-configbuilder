/// Assemble application configuration from named sources.
///
/// The `Sized` bound allows implementors to use `Self` in fluent builder
/// returns. Obtain a concrete instance via the `saf/` factory functions —
/// callers never name the concrete type.
pub trait ConfigBuilder: Sized {
    /// Return the configured application name.
    fn name(&self) -> &str;

    /// Return the configured application version.
    fn version(&self) -> &str;

    /// Set the application name.
    fn with_name(self, name: impl Into<String>) -> Self;

    /// Set the application version string.
    fn with_version(self, version: impl Into<String>) -> Self;
}
