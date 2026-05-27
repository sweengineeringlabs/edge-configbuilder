//! Extension hooks for downstream consumers.
//!
//! Implement [`Loader`] or [`Validator`] to plug in a custom config source or
//! path-validation strategy. Wire it up via [`crate::saf`] factory patterns or
//! inject directly as a generic type parameter.
//!
//! Implement [`ConfigSection`] on a typed struct to make it loadable from a
//! TOML section by the runtime in a single call.
//!
//! Implement [`OptionalSection`] on a typed struct when the section's presence
//! in TOML should enable a feature — absence means the feature is off, no error
//! raised.  Override [`OptionalSection::validate_enabled`] to enforce cross-field
//! constraints that serde cannot express (e.g. "cert_path required when tls = true").
//! Override [`OptionalSection::requires`] to declare feature-level dependencies.
//! Override [`OptionalSection::on_error`] to choose graceful degradation over
//! hard failure when `validate_enabled` rejects an otherwise-present section.
//! Override [`OptionalSection::metadata`] to annotate the feature with description,
//! owner, and deprecation information for richer startup summaries.
//!
//! [`Loader`]: crate::api::loader::traits::loader::Loader
//! [`Validator`]: crate::api::loader::traits::validator::Validator

use crate::api::feature::traits::feature_loader::FeatureLoader;
use crate::api::feature::types::feature_metadata::FeatureMetadata;
use crate::api::feature::types::feature_state::FeatureState;
use crate::api::feature::types::on_error::OnError;
use crate::api::loader::errors::config_error::ConfigError;
use crate::api::loader::traits::loader::Loader;

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
/// # Optional method
///
/// Override `validate_enabled` to enforce cross-field constraints that serde
/// cannot express (e.g. "cert_path is required when tls_enabled = true").
/// The method is only called when the section is present.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(serde::Deserialize)]
/// pub struct MessageBrokerConfig {
///     pub host:        String,
///     pub port:        u16,
///     pub tls_enabled: bool,
///     pub cert_path:   Option<String>,
/// }
///
/// impl swe_edge_configbuilder::OptionalSection for MessageBrokerConfig {
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
/// // runtime startup
/// match MessageBrokerConfig::load_optional(&loader)? {
///     FeatureState::Enabled(cfg) => init_broker(cfg),
///     FeatureState::Disabled     => {}
/// }
/// ```
pub trait OptionalSection: serde::de::DeserializeOwned + Send + Sync + 'static {
    /// The top-level TOML key for this section (e.g. `"message_broker"`).
    fn section_name() -> &'static str; // @allow: no_stub_fn_bodies — required trait method, no default

    /// Other section keys this feature requires to be enabled before it can run.
    ///
    /// Checked by [`FeatureRegistry::validate_dependencies`] after all features
    /// are loaded.  If a required section is absent or disabled, `validate_dependencies`
    /// returns an error listing every violation.
    ///
    /// Default: no dependencies (`&[]`).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// fn requires() -> &'static [&'static str] {
    ///     &["tls", "auth"]
    /// }
    /// ```
    ///
    /// [`FeatureRegistry::validate_dependencies`]: crate::saf::configbuilder_svc::FeatureRegistry::validate_dependencies
    fn requires() -> &'static [&'static str] {
        &[]
    }

    /// What [`FeatureRegistry`] should do when [`validate_enabled`] returns an error.
    ///
    /// - [`OnError::Fail`] *(default)* — propagate the error; startup halts.
    /// - [`OnError::Disable`] — treat the feature as disabled and continue startup;
    ///   the validation error reason is recorded in [`FeatureRecord::override_source`]
    ///   as [`OverrideSource::ValidationError`].
    ///
    /// Override at deploy time via env var
    /// `SWE_EDGE_FEATURE_<UPPER_KEY>_ON_ERROR=fail|disable` (takes precedence over
    /// this method's return value).
    ///
    /// [`FeatureRegistry`]: crate::saf::configbuilder_svc::FeatureRegistry
    /// [`validate_enabled`]: OptionalSection::validate_enabled
    /// [`FeatureRecord::override_source`]: crate::api::feature::types::feature_record::FeatureRecord::override_source
    /// [`OverrideSource::ValidationError`]: crate::api::feature::types::override_source::OverrideSource::ValidationError
    fn on_error() -> OnError {
        OnError::Fail
    }

    /// Static documentation and ownership annotations for this feature.
    ///
    /// Used by [`FeatureSummary`] to produce self-documenting startup output:
    ///
    /// ```text
    /// [ON ] message_broker  — AMQP broker (owner: platform-team)
    /// [OFF] analytics       — [DEPRECATED since 0.4.0]
    /// ```
    ///
    /// Default: all fields empty / `None`.
    ///
    /// [`FeatureSummary`]: crate::saf::configbuilder_svc::FeatureSummary
    fn metadata() -> FeatureMetadata {
        FeatureMetadata::default()
    }

    /// Validate cross-field constraints after the section has been deserialised.
    ///
    /// Only called when the section key is present in TOML.  Return
    /// `Err(ConfigError::validation(Self::section_name(), "...reason..."))` to
    /// reject the section with a clear operator message.
    ///
    /// Default: always `Ok(())` — no additional validation.
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
    /// through [`FeatureRegistry::load`].  Use `FeatureRegistry` for full
    /// startup coordination with graceful degradation and dependency validation.
    ///
    /// [`on_error`]: OptionalSection::on_error
    /// [`requires`]: OptionalSection::requires
    /// [`FeatureRegistry::load`]: crate::saf::configbuilder_svc::FeatureRegistry::load
    fn load_optional(loader: &impl FeatureLoader) -> Result<FeatureState<Self>, ConfigError>
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
