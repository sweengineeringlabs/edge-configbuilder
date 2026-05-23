//! `DefaultConfigBuilder` — default `ConfigBuilder` implementation.

use crate::api::traits::config_builder::ConfigBuilder;

pub(crate) struct DefaultConfigBuilder {
    name: String,
    version: String,
}

impl DefaultConfigBuilder {
    pub(crate) fn new() -> Self {
        Self {
            name: String::new(),
            version: "0.1.0".to_string(),
        }
    }
}

impl ConfigBuilder for DefaultConfigBuilder {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_builder_with_empty_name_and_default_version() {
        let b = DefaultConfigBuilder::new();
        assert_eq!(b.name(), "");
        assert_eq!(b.version(), "0.1.0");
    }

    #[test]
    fn test_with_name_sets_application_name() {
        let b = DefaultConfigBuilder::new().with_name("swe-edge-config");
        assert_eq!(b.name(), "swe-edge-config");
    }

    #[test]
    fn test_with_version_sets_application_version() {
        let b = DefaultConfigBuilder::new().with_version("2.0.0");
        assert_eq!(b.version(), "2.0.0");
    }

    #[test]
    fn test_name_returns_configured_application_name() {
        let b = DefaultConfigBuilder::new().with_name("edge-config");
        assert_eq!(b.name(), "edge-config");
    }

    #[test]
    fn test_version_returns_configured_application_version() {
        let b = DefaultConfigBuilder::new().with_version("1.2.3");
        assert_eq!(b.version(), "1.2.3");
    }
}
