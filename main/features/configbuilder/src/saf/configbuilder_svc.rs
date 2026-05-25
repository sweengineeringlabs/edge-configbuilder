use std::path::PathBuf;

use crate::api::error::config_error::ConfigError;
use crate::api::traits::config_builder::ConfigBuilder;
use crate::api::traits::loader::Loader;
use crate::api::traits::validator::Validator;
use crate::core::DefaultConfigBuilder;
use crate::core::DefaultValidator;

/// Create a loader reading from `SWE_EDGE_CONFIG_DIR`, falling back to `config/`.
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if `SWE_EDGE_CONFIG_DIR` contains `..` traversal
/// components, or if the resolved path exists but is not a directory.
pub fn create_loader() -> Result<impl Loader, ConfigError> {
    DefaultConfigBuilder {
        name: String::new(),
        version: String::new(),
        config_dirs: Vec::new(),
    }
    .build_loader()
}

/// Create a loader reading from a single explicit directory.
pub fn create_loader_for_dir(dir: impl Into<PathBuf>) -> impl Loader {
    crate::core::DefaultSectionLoader {
        config_dirs: vec![dir.into()],
    }
}

/// Create a loader following the XDG Base Directory chain for `app_name`.
///
/// Layer order (last wins):
/// - `$XDG_CONFIG_DIRS/<app_name>/` (lowest priority)
/// - `$XDG_CONFIG_HOME/<app_name>/`
/// - `$SWE_EDGE_CONFIG_DIR/` (if set)
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if any environment-variable-supplied path
/// contains `..` traversal components, or if a resolved path exists but is
/// not a directory.
pub fn create_loader_xdg(app_name: &str) -> Result<impl Loader, ConfigError> {
    DefaultConfigBuilder {
        name: app_name.to_owned(),
        version: String::new(),
        config_dirs: Vec::new(),
    }
    .build_loader()
}

/// Create a path validator.
pub fn create_validator() -> impl Validator {
    DefaultValidator
}

/// Create a config builder pre-seeded with this package's name and version.
///
/// Uses XDG Base Directory resolution for the package name so callers do not
/// need to call [`ConfigBuilder::with_name`] manually.
pub fn create_config_builder() -> impl ConfigBuilder {
    DefaultConfigBuilder {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        config_dirs: Vec::new(),
    }
}
