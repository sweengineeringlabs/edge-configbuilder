use thiserror::Error;

/// Errors returned by config loading and validation operations.
///
/// Returned by [`Loader::load_section`], [`FeatureLoader::load_feature`], and
/// every factory function that builds a loader. Match on the variant to decide
/// whether to abort startup or log and continue.
///
/// [`Loader::load_section`]: crate::Loader::load_section
/// [`FeatureLoader::load_feature`]: crate::FeatureLoader::load_feature
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::ConfigError;
///
/// // Construct a validation error with an actionable message.
/// let err = ConfigError::validation(
///     "auth",
///     "cert_path is required when tls_enabled = true",
/// );
/// assert!(err.to_string().contains("cert_path"));
///
/// // Match on the variant.
/// match err {
///     ConfigError::Validation { section, reason } => {
///         assert_eq!(section, "auth");
///         assert!(reason.contains("cert_path"));
///     }
///     _ => panic!("unexpected variant"),
/// }
/// ```
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

    /// A cross-field or semantic constraint was violated after deserialization.
    ///
    /// Returned when a TOML section is present and parses successfully but
    /// fails the post-deserialization validation defined by
    /// [`OptionalSection::validate_enabled`].
    ///
    /// The message must be actionable: state what constraint was violated and
    /// what the operator must do to resolve it.
    ///
    /// [`OptionalSection::validate_enabled`]: crate::api::traits::optional_section::OptionalSection::validate_enabled
    #[error("config validation error in section '{section}': {reason}")]
    Validation {
        /// The TOML section key where the violation was detected (e.g. `"message_broker"`).
        section: String,
        /// Human-readable description of the constraint that was violated.
        reason: String,
    },
}

impl ConfigError {
    /// Construct a [`ConfigError::Validation`] with the section name and reason.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_configbuilder::ConfigError;
    ///
    /// let err = ConfigError::validation("tls", "key_path is required");
    /// assert!(matches!(err, ConfigError::Validation { .. }));
    /// ```
    pub fn validation(section: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Validation {
            section: section.into(),
            reason: reason.into(),
        }
    }
}
