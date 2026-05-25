//! Extension hooks for downstream consumers.
//!
//! Implement [`Loader`] or [`Validator`] to plug in a custom config source or
//! path-validation strategy. Wire it up via [`crate::saf`] factory patterns or
//! inject directly as a generic type parameter.
//!
//! Implement [`ConfigSection`] on a typed struct to make it loadable from a
//! TOML section by the runtime in a single call — no per-crate builder needed.
//!
//! [`Loader`]: crate::api::traits::loader::Loader
//! [`Validator`]: crate::api::traits::validator::Validator

use crate::api::error::config_error::ConfigError;
use crate::api::traits::loader::Loader;

/// Marks a typed struct as the owner of a named TOML section.
///
/// Implement this on any `serde::Deserialize + Default` config struct. The
/// runtime calls [`ConfigSection::load`] once at startup — callers never
/// write `create_config_builder().build_loader().load_section(...)` manually.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(serde::Deserialize, Default)]
/// pub struct MtlsAuthConfig { pub allowed_cns: Vec<String> }
///
/// impl swe_edge_configbuilder::ConfigSection for MtlsAuthConfig {
///     fn section_name() -> &'static str { "mtls" }
/// }
///
/// // runtime
/// let cfg = MtlsAuthConfig::load(&loader)?;
/// ```
pub trait ConfigSection: serde::de::DeserializeOwned + Default + Send + Sync + 'static {
    /// The top-level TOML key for this section (e.g. `"mtls"`, `"authz"`).
    fn section_name() -> &'static str; // @allow: no_stub_fn_bodies — required trait method, no default

    /// Load this section from `loader`, returning `Self::default()` when the
    /// key is absent. Override only when custom merge logic is required.
    fn load(loader: &impl Loader) -> Result<Self, ConfigError> {
        loader.load_section(Self::section_name())
    }
}
