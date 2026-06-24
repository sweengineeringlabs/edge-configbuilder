//! [`OptionalSection`] — marks a typed struct as an opt-in TOML feature section.

use crate::api::loader::types::feature_metadata::FeatureMetadata;
use crate::api::loader::types::feature_state::FeatureState;
use crate::api::loader::types::on_error::OnError;
use crate::api::loader::types::section_loader_impl::SectionLoaderImpl;
use crate::api::ConfigError;

/// Marks a typed struct as an opt-in TOML feature section.
///
/// Unlike [`ConfigSection`], which returns `T::default()` for absent keys,
/// `OptionalSection` treats absence as "feature disabled" and returns
/// [`FeatureState::Disabled`] — no defaults, no error.
///
/// # Required method
///
/// Implement `section_name()` to declare the top-level TOML key.
///
/// # Optional methods
///
/// Override `validate_enabled` to enforce cross-field constraints after
/// deserialisation (e.g. "cert_path is required when tls_enabled = true").
/// Override `requires` to declare feature-level dependencies.
/// Override `on_error` to choose graceful degradation over hard failure.
/// Override `metadata` to annotate the feature with description, owner, and
/// deprecation information for richer startup summaries.
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_configbuilder::{ConfigError, FeatureState, OptionalSection};
///
/// #[derive(serde::Deserialize)]
/// pub struct MessageBrokerConfig {
///     pub host:        String,
///     pub port:        u16,
///     pub tls_enabled: bool,
///     pub cert_path:   Option<String>,
/// }
///
/// impl OptionalSection for MessageBrokerConfig {
///     fn section_name() -> &'static str { "message_broker" }
///
///     fn validate_enabled(&self) -> Result<(), ConfigError> {
///         if self.tls_enabled && self.cert_path.is_none() {
///             return Err(ConfigError::validation(
///                 Self::section_name(),
///                 "cert_path is required when tls_enabled = true",
///             ));
///         }
///         Ok(())
///     }
/// }
///
/// # let loader: swe_edge_configbuilder::SectionLoaderImpl = panic!();
/// match MessageBrokerConfig::load_optional(&loader).expect("load failed") {
///     FeatureState::Enabled(cfg) => println!("broker at {}:{}", cfg.host, cfg.port),
///     FeatureState::Disabled     => println!("broker not configured"),
/// }
/// ```
///
/// [`ConfigSection`]: crate::ConfigSection
pub trait OptionalSection: serde::de::DeserializeOwned + Send + Sync + 'static {
    /// The top-level TOML key for this section (e.g. `"message_broker"`).
    fn section_name() -> &'static str; // @allow: no_stub_fn_bodies — required trait method, no default

    /// Other section keys this feature requires to be enabled before it can run.
    ///
    /// Checked by [`FeatureRegistry::validate_dependencies`] after all features
    /// are loaded.  Default: no dependencies (`&[]`).
    ///
    /// [`FeatureRegistry::validate_dependencies`]: crate::api::loader::types::feature_registry::FeatureRegistry::validate_dependencies
    fn requires() -> &'static [&'static str] {
        &[]
    }

    /// What [`FeatureRegistry`] should do when [`validate_enabled`] returns an error.
    ///
    /// - [`OnError::Fail`] *(default)* — propagate the error; startup halts.
    /// - [`OnError::Disable`] — treat the feature as disabled and continue startup.
    ///
    /// Override at deploy time via env var
    /// `SWE_EDGE_FEATURE_<UPPER_KEY>_ON_ERROR=fail|disable`.
    ///
    /// [`FeatureRegistry`]: crate::api::loader::types::feature_registry::FeatureRegistry
    /// [`validate_enabled`]: OptionalSection::validate_enabled
    fn on_error() -> OnError {
        OnError::Fail
    }

    /// Static documentation and ownership annotations for this feature.
    ///
    /// Used by [`FeatureSummary`] to produce self-documenting startup output.
    /// Default: all fields empty / `None`.
    ///
    /// [`FeatureSummary`]: crate::api::loader::types::feature_summary::FeatureSummary
    fn metadata() -> FeatureMetadata {
        FeatureMetadata::default()
    }

    /// Validate cross-field constraints after the section has been deserialised.
    ///
    /// Only called when the section key is present in TOML. Default: always `Ok(())`.
    fn validate_enabled(&self) -> Result<(), ConfigError> {
        Ok(())
    }

    /// Load this section as an optional feature from `loader`.
    ///
    /// Returns `FeatureState::Enabled(Self)` when the key is present and valid,
    /// `FeatureState::Disabled` when absent, and `Err` for I/O, parse, or
    /// validation failures.
    ///
    /// **Note:** [`on_error`] and [`requires`] are only applied when loading
    /// through [`FeatureRegistry::load`]. Use `FeatureRegistry` for full
    /// startup coordination with graceful degradation and dependency validation.
    ///
    /// [`on_error`]: OptionalSection::on_error
    /// [`requires`]: OptionalSection::requires
    /// [`FeatureRegistry::load`]: crate::api::loader::types::feature_registry::FeatureRegistry::load
    fn load_optional(loader: &SectionLoaderImpl) -> Result<FeatureState<Self>, ConfigError>
    where
        Self: Sized,
    {
        let state: FeatureState<Self> = loader.load_optional_section(Self::section_name())?;
        if let FeatureState::Enabled(ref value) = state {
            value.validate_enabled()?;
        }
        Ok(state)
    }
}
