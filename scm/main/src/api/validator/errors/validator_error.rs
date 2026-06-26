use thiserror::Error;

/// Errors returned by path validation ports.
#[derive(Debug, Error)]
pub enum ValidatorError {
    /// Path exists but is not a directory, or another I/O failure occurred.
    #[error("config io error: {0}")]
    Io(String),
}
