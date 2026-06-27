//! Public concrete section loader returned by the `saf/` factory functions.

use crate::{ConfigError, FeatureLoader, FeatureState, LoadedFeature, Loader, SectionLoaderImpl};

impl SectionLoaderImpl {
    /// Load and deserialize a named section.
    pub(crate) fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let val = self.ops.load_section_value(key)?;
        if val.as_table().is_some_and(|t| t.is_empty()) {
            return Ok(T::default());
        }
        val.try_into()
            .map_err(|e: toml::de::Error| ConfigError::Parse(e.to_string()))
    }

    /// Validate the loader's configured directories.
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        self.ops.validate_dirs()
    }

    /// Load a named feature and return its state plus record metadata.
    pub(crate) fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
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

    /// Load a named section and return only its enabled/disabled state.
    pub(crate) fn load_optional_section<T>(&self, key: &str) -> Result<FeatureState<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.load_feature(key).map(|loaded| loaded.state)
    }
}

impl Loader for crate::SectionLoaderImpl {
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

impl FeatureLoader for crate::SectionLoaderImpl {
    fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.load_feature(key)
    }
}
