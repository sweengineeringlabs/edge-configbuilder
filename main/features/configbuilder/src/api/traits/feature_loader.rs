//! [`FeatureLoader`] — presence-aware section loading with full metadata.

use crate::api::error::config_error::ConfigError;
use crate::api::types::feature::feature_state::FeatureState;
use crate::api::types::feature::loaded_feature::LoadedFeature;

/// Load optional TOML sections with configurable enable/disable precedence.
///
/// Unlike [`Loader`], which returns `T::default()` for absent sections,
/// `FeatureLoader` distinguishes absent (feature off) from present (feature on)
/// and enforces a three-level precedence chain:
///
/// ```text
/// env var  >  enabled = false  >  section presence  >  section absent
/// ```
///
/// ## Control mechanisms
///
/// | Mechanism | How to use | Example |
/// |-----------|------------|---------|
/// | Section presence | Add/remove `[section]` in TOML | `[message_broker]` |
/// | Explicit disable | Set `enabled = false` inside the section | `enabled = false` |
/// | Env-var override | Set `SWE_EDGE_FEATURE_<UPPER_KEY>=false/true` | `SWE_EDGE_FEATURE_MESSAGE_BROKER=false` |
///
/// ## Methods
///
/// - [`load_feature`] — full result including metadata record (use this with [`FeatureRegistry`])
/// - [`load_optional_section`] — state only, no metadata (use this for ad-hoc loading)
///
/// [`Loader`]: crate::api::traits::loader::Loader
/// [`load_feature`]: FeatureLoader::load_feature
/// [`load_optional_section`]: FeatureLoader::load_optional_section
/// [`FeatureRegistry`]: crate::saf::configbuilder_svc::FeatureRegistry
pub trait FeatureLoader {
    /// Load the section at `key` with full metadata.
    ///
    /// Applies the full precedence chain — env var override, explicit
    /// `enabled = false`, section presence — and records the reason in the
    /// returned [`LoadedFeature::record`].
    ///
    /// Prefer [`load_optional_section`] for simple ad-hoc loading.
    /// Use this method when you need the metadata (e.g. building a
    /// [`FeatureRegistry`] startup report).
    ///
    /// # Returns
    ///
    /// - `Ok(LoadedFeature { state: Enabled(T), .. })` — section present, all
    ///   controls say on.
    /// - `Ok(LoadedFeature { state: Disabled, .. })` — section absent, env var
    ///   says off, or `enabled = false` set in TOML.
    /// - `Err(ConfigError::Io)` — unreadable file or 1 MiB size limit exceeded.
    /// - `Err(ConfigError::Parse)` — malformed TOML or deserialisation failure.
    ///
    /// [`load_optional_section`]: FeatureLoader::load_optional_section
    /// [`FeatureRegistry`]: crate::saf::configbuilder_svc::FeatureRegistry
    fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned;

    /// Load the section at `key` as a `FeatureState`, without metadata.
    ///
    /// A convenience wrapper over [`load_feature`] that discards the
    /// [`FeatureRecord`].  Use this for one-off loading; use [`load_feature`]
    /// or [`FeatureRegistry`] when you need observability.
    ///
    /// [`load_feature`]: FeatureLoader::load_feature
    /// [`FeatureRecord`]: crate::api::types::feature::feature_record::FeatureRecord
    /// [`FeatureRegistry`]: crate::saf::configbuilder_svc::FeatureRegistry
    fn load_optional_section<T>(&self, key: &str) -> Result<FeatureState<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.load_feature(key).map(|loaded| loaded.state)
    }
}
