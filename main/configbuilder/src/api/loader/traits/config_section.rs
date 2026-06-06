//! [`ConfigSection`] — marks a typed struct as the owner of a named TOML section.

use crate::api::error::config_error::ConfigError;
use crate::api::loader::types::section_loader_impl::SectionLoaderImpl;

/// Marks a typed struct as the owner of a named TOML section.
///
/// Implement this on any `serde::Deserialize + Default` config struct. The
/// runtime calls [`ConfigSection::load`] once at startup — callers never
/// write `ConfigLoaderFactory::create_config_builder().build_loader().load_section(...)` manually.
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_configbuilder::{ConfigLoaderFactory, ConfigSection};
///
/// #[derive(serde::Deserialize, Default)]
/// pub struct MtlsAuthConfig { pub allowed_cns: Vec<String> }
///
/// impl ConfigSection for MtlsAuthConfig {
///     fn section_name() -> &'static str { "mtls" }
/// }
///
/// let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
/// let cfg = MtlsAuthConfig::load(&loader).expect("mtls section required");
/// ```
pub trait ConfigSection: serde::de::DeserializeOwned + Default + Send + Sync + 'static {
    /// The top-level TOML key for this section (e.g. `"mtls"`, `"authz"`).
    fn section_name() -> &'static str; // @allow: no_stub_fn_bodies — required trait method, no default

    /// Load this section from `loader`, returning `Self::default()` when the
    /// key is absent. Override only when custom merge logic is required.
    fn load(loader: &SectionLoaderImpl) -> Result<Self, ConfigError> {
        loader.load_section(Self::section_name())
    }
}
