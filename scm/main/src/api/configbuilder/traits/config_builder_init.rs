use std::time::Duration;

/// Initialisation and timeout-override methods for concrete config builders.
///
/// Implemented by [`ConfigBuilderImpl`] in the `core/` layer.
///
/// [`ConfigBuilderImpl`]: crate::ConfigBuilderImpl
pub trait ConfigBuilderInit: Sized {
    /// Create an empty builder with no name, version, or config dirs set.
    fn new() -> Self;

    /// Override the default 30-second read deadline for each `application.toml`.
    fn with_read_timeout(self, timeout: Duration) -> Self;
}
