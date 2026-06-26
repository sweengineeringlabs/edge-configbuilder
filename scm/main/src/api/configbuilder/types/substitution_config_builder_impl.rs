//! Public concrete substitution config builder returned by
//! `create_config_builder_with_substitution`.

use crate::api::substitution::traits::substitution_policy::SubstitutionPolicy;
use std::path::PathBuf;

/// A ready-to-use config builder with substitution support, produced by
/// [`ConfigLoaderFactory::create_config_builder_with_substitution`].
///
/// Use the fluent builder methods to configure directories, then call
/// `build_loader` to obtain a [`SectionLoaderImpl`] that will expand `{{VAR}}`
/// placeholders in TOML values using the bound [`SubstitutionPolicy`].
///
/// The `build_loader` method is provided by an extension impl in `saf/` so
/// that this type carries no dependency on `core/` (SEA rules 46 and 116).
///
/// [`ConfigLoaderFactory::create_config_builder_with_substitution`]: crate::ConfigLoaderFactory::create_config_builder_with_substitution
/// [`SectionLoaderImpl`]: crate::SectionLoaderImpl
/// [`SubstitutionPolicy`]: crate::SubstitutionPolicy
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_configbuilder::ConfigLoaderFactory;
///
/// #[derive(serde::Deserialize, Default)]
/// struct DbConfig { url: String }
///
/// // TOML: url = "postgres://{{DB_USER}}:{{DB_PASS}}@host/db"
/// let loader = ConfigLoaderFactory::create_config_builder_with_substitution(
///         Box::new(ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
///             "APP_".to_string()
///         ])),
///     )
///     .with_config_dir("config/")
///     .build_loader()
///     .expect("config dir accessible");
///
/// let cfg: DbConfig = loader.load_section("database").expect("database section required");
/// // cfg.url has had {{DB_USER}} and {{DB_PASS}} substituted.
/// ```
pub struct SubstitutionConfigBuilderImpl {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) config_dirs: Vec<PathBuf>,
    pub(crate) policy: Box<dyn SubstitutionPolicy>,
}
