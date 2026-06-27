//! [`FeatureRegistry`] — startup feature collector and dependency validator.

use crate::api::{
    ConfigError, FeatureMetadata, FeatureRecord, FeatureRecordBuilder, FeatureRegistry,
    FeatureState, LoadedFeature, OnError, OptionalSection, OverrideSource, SectionLoaderImpl,
};

impl FeatureRegistry {
    /// Create an empty registry.
    pub(crate) fn new() -> Self {
        Self {
            records: Vec::new(),
            observers: Vec::new(),
        }
    }

    /// Register a callback to observe loaded records.
    pub(crate) fn on_load(&mut self, observer: impl Fn(&FeatureRecord) + 'static) {
        self.observers.push(Box::new(observer));
    }

    /// Load a feature section and record its state.
    pub(crate) fn load<T>(&mut self, loader: &SectionLoaderImpl) -> Result<FeatureState<T>, ConfigError>
    where
        T: OptionalSection,
    {
        let loaded: LoadedFeature<T> = loader.load_feature(T::section_name())?;
        let LoadedFeature { state, record } = loaded;
        let record = *record;

        let validation_result = if let FeatureState::Enabled(ref value) = state {
            Some(value.validate_enabled())
        } else {
            None
        };

        let (final_state, final_override) = match validation_result {
            Some(Ok(())) | None => (state, record.override_source),
            Some(Err(e)) => match crate::core::DefaultSectionLoader::resolve_feature_on_error::<T>(
                T::section_name(),
            ) {
                OnError::Fail => return Err(e),
                OnError::Disable => (
                    FeatureState::Disabled,
                    Some(OverrideSource::ValidationError {
                        reason: e.to_string(),
                    }),
                ),
            },
        };

        let mut built = FeatureRecordBuilder::new(record.section_name)
            .enabled(final_state.is_enabled())
            .requires(T::requires())
            .metadata(T::metadata());
        if let Some(override_source) = final_override {
            built = built.override_source(override_source);
        }
        self.records.push(built.build());

        if let Some(record) = self.records.last() {
            for observer in &self.observers {
                observer(record);
            }
        }

        Ok(final_state)
    }

    /// Validate recorded dependencies.
    pub(crate) fn validate_dependencies(&self) -> Result<(), ConfigError> {
        let enabled: std::collections::HashSet<&str> = self
            .records
            .iter()
            .filter(|r| r.enabled)
            .map(|r| r.section_name.as_str())
            .collect();

        let violations: Vec<String> = self
            .records
            .iter()
            .filter(|r| r.enabled)
            .flat_map(|r| {
                r.requires.iter().filter_map(|dep| {
                    if enabled.contains(dep) {
                        None
                    } else {
                        Some(format!(
                            "'{}' requires '{}' but '{}' is not enabled",
                            r.section_name, dep, dep
                        ))
                    }
                })
            })
            .collect();

        if violations.is_empty() {
            Ok(())
        } else {
            Err(ConfigError::Validation {
                section: String::from("feature_dependencies"),
                reason: violations.join("; "),
            })
        }
    }

    /// Borrow the recorded feature records.
    pub(crate) fn records(&self) -> &[FeatureRecord] {
        &self.records
    }

    /// Build a snapshot summary of the recorded features.
    pub(crate) fn summary(&self) -> crate::FeatureSummary {
        crate::FeatureSummary {
            records: self.records.clone(),
        }
    }
}

impl crate::api::FeatureRegistryOps for FeatureRegistry {
    fn new() -> Self {
        FeatureRegistry::new()
    }

    fn on_load(&mut self, observer: impl Fn(&FeatureRecord) + 'static) {
        FeatureRegistry::on_load(self, observer)
    }

    fn load<T>(&mut self, loader: &SectionLoaderImpl) -> Result<FeatureState<T>, ConfigError>
    where
        T: OptionalSection,
    {
        FeatureRegistry::load(self, loader)
    }

    fn validate_dependencies(&self) -> Result<(), ConfigError> {
        FeatureRegistry::validate_dependencies(self)
    }

    fn records(&self) -> &[FeatureRecord] {
        FeatureRegistry::records(self)
    }

    fn summary(&self) -> crate::FeatureSummary {
        FeatureRegistry::summary(self)
    }
}

impl Default for FeatureRegistry {
    fn default() -> Self {
        Self::new()
    }
}
