use std::path::PathBuf;

/// Assemble application configuration from named sources.
///
/// `ConfigBuilder` is a **builder-chain trait** — it covers only the configuration
/// fields (`name`, `version`, `with_config_dir`).  It deliberately does **not**
/// include `build_loader()`.  That finaliser is an inherent method on the concrete
/// types ([`ConfigBuilderImpl`], [`SubstitutionConfigBuilderImpl`]) because the
/// construction logic depends on `core/` internals that the `api/` trait layer must
/// not reference (SEA rules 46 and 116).
///
/// # Two usage patterns
///
/// | Return type | Can call `build_loader()`? | When to use |
/// |---|---|---|
/// | `ConfigBuilderImpl` (concrete) | **yes** | SAF entry points — callers need to finalise into a loader |
/// | `impl ConfigBuilder` (opaque) | **no** | Intermediate helpers that receive a partially-built builder and add fields, then hand it back to the caller who holds the concrete type |
///
/// **SAF `create_config_builder()` functions must return `ConfigBuilderImpl`**, not
/// `impl ConfigBuilder`.  Returning the opaque trait type would prevent callers from
/// ever calling `build_loader()` to actually load config.
///
/// # Obtaining a concrete instance
///
/// ```rust,no_run
/// use swe_edge_configbuilder::ConfigLoaderFactory;
///
/// let loader = ConfigLoaderFactory::create_config_builder()
///     .with_name("my-service")
///     .with_version("1.0.0")
///     .build_loader()
///     .expect("config dir accessible");
/// ```
///
/// [`ConfigBuilderImpl`]: crate::api::types::config::ConfigBuilderImpl
/// [`SubstitutionConfigBuilderImpl`]: crate::api::types::substitution_config_builder_impl::SubstitutionConfigBuilderImpl
pub trait ConfigBuilder: Sized {
    /// Return the configured application name.
    fn name(&self) -> &str;

    /// Return the configured application version.
    fn version(&self) -> &str;

    /// Set the application name; used by `build_loader` to resolve XDG paths.
    fn with_name(self, name: impl Into<String>) -> Self;

    /// Set the application version string.
    fn with_version(self, version: impl Into<String>) -> Self;

    /// Append an explicit config directory; takes precedence over XDG resolution.
    ///
    /// Multiple calls accumulate directories — later entries win on key conflicts.
    fn with_config_dir(self, dir: impl Into<PathBuf>) -> Self;
}
