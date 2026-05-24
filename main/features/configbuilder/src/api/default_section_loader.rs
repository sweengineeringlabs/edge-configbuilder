//! Interface contract for [`crate::core::DefaultSectionLoader`].

/// Maximum size of a single `application.toml` file accepted by the loader.
pub(crate) const MAX_CONFIG_FILE_BYTES: u64 = 1_048_576;
