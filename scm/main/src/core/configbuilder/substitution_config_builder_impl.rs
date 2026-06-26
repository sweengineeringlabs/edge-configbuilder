use crate::api::{ConfigBuilder, ConfigBuilderImpl, SubstitutionConfigBuilderImpl};

impl ConfigBuilder for SubstitutionConfigBuilderImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn with_name(self, name: impl Into<String>) -> Self {
        SubstitutionConfigBuilderImpl::with_name(self, name)
    }

    fn with_version(self, version: impl Into<String>) -> Self {
        SubstitutionConfigBuilderImpl::with_version(self, version)
    }

    fn with_config_dir(self, dir: impl Into<std::path::PathBuf>) -> Self {
        SubstitutionConfigBuilderImpl::with_config_dir(self, dir)
    }
}

impl From<SubstitutionConfigBuilderImpl> for ConfigBuilderImpl {
    fn from(value: SubstitutionConfigBuilderImpl) -> Self {
        ConfigBuilderImpl {
            name: value.name,
            version: value.version,
            config_dirs: value.config_dirs,
            read_timeout: None,
        }
    }
}
