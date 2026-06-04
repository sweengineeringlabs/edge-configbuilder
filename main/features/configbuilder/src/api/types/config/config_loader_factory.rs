//! [`ConfigLoaderFactory`] — zero-size factory type for constructing config loaders.

/// Factory for creating configuration section loaders and config builders.
///
/// All construction is via associated functions — no instantiation required.
/// Import the type and call the factory method that matches your use case:
///
/// | Factory method | When to use |
/// |---|---|
/// | [`create_loader`] | Default: reads `SWE_EDGE_CONFIG_DIR` or `config/` |
/// | [`create_loader_for_dir`] | You know the exact directory at call time |
/// | [`create_loader_xdg`] | XDG multi-directory resolution for a named app |
/// | [`create_config_builder`] | You want to chain `.with_name()` / `.with_config_dir()` |
/// | `create_loader_*_with_substitution` | Same as above + `{{VAR}}` expansion |
///
/// [`create_loader`]: ConfigLoaderFactory::create_loader
/// [`create_loader_for_dir`]: ConfigLoaderFactory::create_loader_for_dir
/// [`create_loader_xdg`]: ConfigLoaderFactory::create_loader_xdg
/// [`create_config_builder`]: ConfigLoaderFactory::create_config_builder
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_configbuilder::ConfigLoaderFactory;
///
/// #[derive(serde::Deserialize, Default)]
/// struct AppConfig { log_level: String }
///
/// // Simplest path: read from default directory.
/// let loader = ConfigLoaderFactory::create_loader().expect("config dir accessible");
/// let cfg: AppConfig = loader.load_section("app").expect("app section required");
/// ```
pub struct ConfigLoaderFactory;
