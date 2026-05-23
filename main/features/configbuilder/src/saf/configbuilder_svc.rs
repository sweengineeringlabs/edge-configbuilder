use std::path::PathBuf;

use crate::api::traits::config_builder::ConfigBuilder;
use crate::api::traits::loader::Loader;
use crate::api::traits::validator::Validator;
use crate::core::ApplicationConfigBuilder;
use crate::core::DefaultConfigBuilder;
use crate::core::DefaultSectionLoader;
use crate::core::DefaultValidator;

/// Create a loader reading from `SWE_EDGE_CONFIG_DIR`, falling back to `config/`.
pub fn create_loader() -> impl Loader {
    DefaultSectionLoader::new()
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
    DefaultSectionLoader::xdg(app_name)
}

/// Create a path validator.
pub fn create_validator() -> impl Validator {
    DefaultValidator
}

/// Create an application config builder.
pub fn create_config_builder() -> impl ConfigBuilder {
    DefaultConfigBuilder::new()
}

/// Create an application config builder pre-seeded with this package's name and version.
///
/// Uses XDG Base Directory resolution for the package name so callers do not
/// need to call [`ConfigBuilder::with_name`] manually.
pub fn create_application_config_builder() -> impl ConfigBuilder {
    ApplicationConfigBuilder::new()
}
