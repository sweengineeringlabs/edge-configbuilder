//! Public concrete section loader returned by the `saf/` factory functions.

use crate::api::error::config_error::ConfigError;
use crate::api::traits::loader_ops::LoaderOps;
use crate::api::types::feature::feature_state::FeatureState;
use crate::api::types::feature::loaded_feature::LoadedFeature;

/// A ready-to-use section loader produced by the `create_loader*` and
/// `ConfigBuilderImpl::build_loader` factory functions.
///
/// Call the inherent methods directly — the [`Loader`] and [`FeatureLoader`]
/// traits do not need to be in scope.
///
/// [`Loader`]: crate::api::traits::loader::Loader
/// [`FeatureLoader`]: crate::api::traits::feature_loader::FeatureLoader
pub struct SectionLoaderImpl {
    pub(crate) ops: Box<dyn LoaderOps>,
}

impl SectionLoaderImpl {
    /// Load the section at `key` (dotted path, e.g. `"outer.inner"`) from all
    /// configured directories.
    ///
    /// If the loader was created with a substitution policy, `{{VAR_NAME}}`
    /// placeholders will be substituted with environment variable values after
    /// loading.
    pub fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let val = self.ops.load_section_value(key)?;
        // Empty table == section absent from all files but files were found;
        // return the type default (same contract as DefaultSectionLoader).
        if val.as_table().map_or(false, |t| t.is_empty()) {
            return Ok(T::default());
        }
        val.try_into()
            .map_err(|e: toml::de::Error| ConfigError::Parse(e.to_string()))
    }

    /// Validate that all configured directories are accessible.
    ///
    /// Non-existent paths are permitted; a path that exists but is not a
    /// directory is an error.
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.ops.validate_dirs()
    }

    /// Load the section at `key` with full metadata.
    pub fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        let raw = self.ops.load_feature_raw(key)?;
        let state = match raw.value {
            None => FeatureState::Disabled,
            Some(val) => FeatureState::Enabled(
                val.try_into()
                    .map_err(|e: toml::de::Error| ConfigError::Parse(e.to_string()))?,
            ),
        };
        Ok(LoadedFeature {
            state,
            record: raw.record,
        })
    }

    /// Load the section at `key` as a `FeatureState`, without metadata.
    pub fn load_optional_section<T>(&self, key: &str) -> Result<FeatureState<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.load_feature(key).map(|loaded| loaded.state)
    }
}

impl crate::api::traits::loader::Loader for SectionLoaderImpl {
    fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        self.load_section(key)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        self.validate()
    }
}

impl crate::api::traits::feature_loader::FeatureLoader for SectionLoaderImpl {
    fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.load_feature(key)
    }
}
