//! `ApplicationConfigBuilder` — package-seeded `ConfigBuilder` implementation.

use std::path::PathBuf;

use crate::api::application_config_builder::{APP_NAME, APP_VERSION};
use crate::api::traits::config_builder::ConfigBuilder;
use crate::api::traits::loader::Loader;
use crate::core::DefaultConfigBuilder;

/// A [`ConfigBuilder`] pre-seeded with this package's name and version.
///
/// Prefer this over building a blank [`ConfigBuilder`] when you want XDG
/// resolution to use the standard swe-edge-configbuilder identity without
/// calling [`ConfigBuilder::with_name`] manually.
pub(crate) struct ApplicationConfigBuilder {
    inner: DefaultConfigBuilder,
}

impl ApplicationConfigBuilder {
    pub(crate) fn new() -> Self {
        Self {
            inner: DefaultConfigBuilder::new()
                .with_name(APP_NAME)
                .with_version(APP_VERSION),
        }
    }
}

impl ConfigBuilder for ApplicationConfigBuilder {
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn version(&self) -> &str {
        self.inner.version()
    }

    fn with_name(mut self, name: impl Into<String>) -> Self {
        self.inner = self.inner.with_name(name);
        self
    }

    fn with_version(mut self, version: impl Into<String>) -> Self {
        self.inner = self.inner.with_version(version);
        self
    }

    fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.inner = self.inner.with_config_dir(dir);
        self
    }

    fn build_loader(self) -> impl Loader {
        self.inner.build_loader()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_uses_package_name_as_default_application_name() {
        let b = ApplicationConfigBuilder::new();
        assert_eq!(b.name(), APP_NAME);
    }

    #[test]
    fn test_new_uses_package_version_as_default_application_version() {
        let b = ApplicationConfigBuilder::new();
        assert_eq!(b.version(), APP_VERSION);
    }

    #[test]
    fn test_with_name_overrides_default_application_name() {
        let b = ApplicationConfigBuilder::new().with_name("custom-app");
        assert_eq!(b.name(), "custom-app");
    }

    #[test]
    fn test_with_version_overrides_default_application_version() {
        let b = ApplicationConfigBuilder::new().with_version("9.9.9");
        assert_eq!(b.version(), "9.9.9");
    }

    #[derive(Debug, Default, serde::Deserialize, PartialEq)]
    #[serde(default)]
    struct Sec {
        value: String,
    }

    #[test]
    fn test_build_loader_returns_usable_loader_for_absent_section() {
        let result: Result<Sec, _> = ApplicationConfigBuilder::new()
            .build_loader()
            .load_section("nonexistent_xyz");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Sec::default());
    }
}
