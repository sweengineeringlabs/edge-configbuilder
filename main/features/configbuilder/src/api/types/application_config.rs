//! [`ApplicationConfig`] — root configuration type for the configbuilder runtime section.

/// Root configuration read from `config/application.toml` under the `[configbuilder]` key.
///
/// All fields are optional; absent keys fall back to environment-variable resolution
/// or XDG defaults. Consumers do not construct this directly — the runtime loads it
/// via the `ConfigSection` impl.
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ApplicationConfig {
    /// Override the default config directory path (normally resolved from XDG or
    /// `SWE_EDGE_CONFIG_DIR`). Leave empty to use the default resolution chain.
    pub config_dir: Option<String>,
}
