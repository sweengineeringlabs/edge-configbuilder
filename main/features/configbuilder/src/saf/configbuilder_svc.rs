use std::env;
use std::path::PathBuf;

use crate::api::traits::config_builder::ConfigBuilder;
use crate::api::traits::loader::Loader;
use crate::api::traits::validator::Validator;
use crate::core::DefaultConfigBuilder;
use crate::core::DefaultSectionLoader;
use crate::core::DefaultValidator;

/// Create a loader reading from `SWE_EDGE_CONFIG_DIR`, falling back to `config/`.
pub fn create_loader() -> impl Loader {
    let dir = env::var("SWE_EDGE_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("config"));
    DefaultSectionLoader {
        config_dirs: vec![dir],
    }
}

/// Create a loader reading from a single explicit directory.
pub fn create_loader_for_dir(dir: impl Into<PathBuf>) -> impl Loader {
    DefaultSectionLoader {
        config_dirs: vec![dir.into()],
    }
}

/// Create a loader following the XDG Base Directory chain for `app_name`.
///
/// Layer order (last wins):
/// - `$XDG_CONFIG_DIRS/<app_name>/` (lowest priority)
/// - `$XDG_CONFIG_HOME/<app_name>/`
/// - `$SWE_EDGE_CONFIG_DIR/` (if set)
pub fn create_loader_xdg(app_name: &str) -> impl Loader {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let xdg_config_dirs = env::var("XDG_CONFIG_DIRS").unwrap_or_else(|_| "/etc/xdg".to_owned());
    for segment in xdg_config_dirs.split(':').rev() {
        if !segment.is_empty() {
            dirs.push(PathBuf::from(segment).join(app_name));
        }
    }
    if let Some(home) = dirs::config_dir() {
        dirs.push(home.join(app_name));
    }
    if let Ok(v) = env::var("SWE_EDGE_CONFIG_DIR") {
        dirs.push(PathBuf::from(v));
    }
    DefaultSectionLoader { config_dirs: dirs }
}

/// Create a path validator.
pub fn create_validator() -> impl Validator {
    DefaultValidator
}

/// Create an application config builder.
pub fn create_config_builder() -> impl ConfigBuilder {
    DefaultConfigBuilder {
        name: String::new(),
        version: "0.1.0".to_string(),
        config_dirs: Vec::new(),
    }
}

/// Create an application config builder pre-seeded with this package's name and version.
///
/// Uses XDG Base Directory resolution for the package name so callers do not
/// need to call [`ConfigBuilder::with_name`] manually.
pub fn create_application_config_builder() -> impl ConfigBuilder {
    DefaultConfigBuilder {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        config_dirs: Vec::new(),
    }
}
