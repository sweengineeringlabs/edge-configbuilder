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
}
