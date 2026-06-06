//! [`ApplicationConfig`] — root configuration type for the configbuilder runtime section.

/// Root configuration read from `config/application.toml` under the `[configbuilder]` key.
///
/// All fields are optional; absent keys fall back to environment-variable resolution
/// or XDG defaults. Consumers do not construct this directly — the runtime loads it
/// via the `ConfigSection` impl.
///
/// # Examples
///
/// ```rust
/// use swe_edge_configbuilder::ApplicationConfig;
///
/// // Default: no override, uses XDG / SWE_EDGE_CONFIG_DIR resolution.
/// let cfg = ApplicationConfig::default();
/// assert!(cfg.config_dir.is_none());
///
/// // Override: point at a specific directory.
/// let cfg = ApplicationConfig { config_dir: Some("/etc/my-app".to_string()) };
/// assert_eq!(cfg.config_dir.as_deref(), Some("/etc/my-app"));
/// ```
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ApplicationConfig {
    /// Override the default config directory path (normally resolved from XDG or
    /// `SWE_EDGE_CONFIG_DIR`). Leave `None` to use the default resolution chain.
    pub config_dir: Option<String>,
}
