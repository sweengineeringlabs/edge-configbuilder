use thiserror::Error;

/// Errors returned by config loading and validation operations.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// A TOML file could not be parsed.
    #[error("config parse error: {0}")]
    Parse(String),

    /// An I/O or filesystem constraint was violated.
    #[error("config io error: {0}")]
    Io(String),

    /// No `application.toml` was found in any configured directory.
    ///
    /// Returned by [`crate::api::traits::loader::Loader::load_section`] when
    /// every candidate directory either does not exist or contains no
    /// `application.toml` file. This usually indicates a misconfigured config
    /// path rather than an intentionally absent section.
    #[error("config not found: {0}")]
    NotFound(String),
}
