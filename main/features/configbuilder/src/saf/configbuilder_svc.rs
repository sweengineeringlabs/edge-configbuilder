use std::env;
use std::path::PathBuf;

use crate::api::default_config_builder::DEFAULT_VERSION;
use crate::api::error::config_error::ConfigError;
use crate::api::traits::config_builder::ConfigBuilder;
use crate::api::traits::loader::Loader;
use crate::api::traits::validator::Validator;
use crate::core::reject_traversal;
use crate::core::DefaultConfigBuilder;
use crate::core::DefaultSectionLoader;
use crate::core::DefaultValidator;

/// Create a loader reading from `SWE_EDGE_CONFIG_DIR`, falling back to `config/`.
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if `SWE_EDGE_CONFIG_DIR` contains `..` traversal
/// components, or if the resolved path exists but is not a directory.
pub fn create_loader() -> Result<impl Loader, ConfigError> {
    let dir = match env::var("SWE_EDGE_CONFIG_DIR") {
        Ok(v) => {
            let p = PathBuf::from(v);
            reject_traversal(&p)?;
            p
        }
        Err(_) => PathBuf::from("config"),
    };
    let loader = DefaultSectionLoader {
        config_dirs: vec![dir],
    };
    loader.validate()?;
    Ok(loader)
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
///
/// # Errors
///
/// Returns [`ConfigError::Io`] if any environment-variable-supplied path
/// contains `..` traversal components, or if a resolved path exists but is
/// not a directory.
pub fn create_loader_xdg(app_name: &str) -> Result<impl Loader, ConfigError> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let xdg_config_dirs = env::var("XDG_CONFIG_DIRS").unwrap_or_else(|_| "/etc/xdg".to_owned());
    for segment in xdg_config_dirs.split(':').rev() {
        if !segment.is_empty() {
            let seg_path = PathBuf::from(segment);
            reject_traversal(&seg_path)?;
            dirs.push(seg_path.join(app_name));
        }
    }
    if let Some(home) = dirs::config_dir() {
        dirs.push(home.join(app_name));
    }
    if let Ok(v) = env::var("SWE_EDGE_CONFIG_DIR") {
        let p = PathBuf::from(&v);
        reject_traversal(&p)?;
        dirs.push(p);
    }
    let loader = DefaultSectionLoader { config_dirs: dirs };
    loader.validate()?;
    Ok(loader)
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
