/// Maximum size, in bytes, of a config file the loader will read.
///
/// Files larger than this are rejected with a [`crate::api::error::config_error::ConfigError::Io`]
/// error.
pub const MAX_CONFIG_FILE_BYTES: u64 = 1_048_576;

/// Environment variable that, when set, overrides the config directory.
pub const CONFIG_DIR_ENV_VAR: &str = "SWE_EDGE_CONFIG_DIR";

/// Relative directory used as fallback when [`CONFIG_DIR_ENV_VAR`] is not set.
pub const FALLBACK_CONFIG_DIR: &str = "config";
