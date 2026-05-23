//! `DefaultConfigBuilder` — default `ConfigBuilder` implementation.

use std::path::PathBuf;

use crate::api::traits::config_builder::ConfigBuilder;
use crate::api::traits::loader::Loader;
use crate::core::DefaultSectionLoader;

pub(crate) struct DefaultConfigBuilder {
    name: String,
    version: String,
    config_dirs: Vec<PathBuf>,
}

impl DefaultConfigBuilder {
    pub(crate) fn new() -> Self {
        Self {
            name: String::new(),
            version: "0.1.0".to_string(),
            config_dirs: Vec::new(),
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

    fn with_config_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.config_dirs.push(dir.into());
        self
    }

    fn build_loader(self) -> impl Loader {
        if !self.config_dirs.is_empty() {
            DefaultSectionLoader {
                config_dirs: self.config_dirs,
            }
        } else if !self.name.is_empty() {
            DefaultSectionLoader::xdg(&self.name)
        } else {
            DefaultSectionLoader::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

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
    fn test_with_config_dir_appends_to_config_dirs() {
        let b = DefaultConfigBuilder::new().with_config_dir("/some/path");
        assert_eq!(b.config_dirs, vec![PathBuf::from("/some/path")]);
    }

    #[test]
    fn test_with_config_dir_multiple_calls_accumulate() {
        let b = DefaultConfigBuilder::new()
            .with_config_dir("/a")
            .with_config_dir("/b");
        assert_eq!(
            b.config_dirs,
            vec![PathBuf::from("/a"), PathBuf::from("/b")]
        );
    }

    #[derive(Debug, Default, serde::Deserialize, PartialEq)]
    #[serde(default)]
    struct Sec {
        value: String,
    }

    #[test]
    fn test_build_loader_with_explicit_dir_reads_application_toml() {
        let dir = tempfile::tempdir().unwrap();
        let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
        writeln!(f, "[svc]\nvalue = \"ok\"").unwrap();
        let sec: Sec = DefaultConfigBuilder::new()
            .with_config_dir(dir.path())
            .build_loader()
            .load_section("svc")
            .unwrap();
        assert_eq!(sec.value, "ok");
    }

    #[test]
    fn test_build_loader_with_name_only_returns_usable_loader() {
        let sec: Sec = DefaultConfigBuilder::new()
            .with_name("swe-edge-nonexistent-test-xyz")
            .build_loader()
            .load_section("any")
            .unwrap();
        assert_eq!(sec, Sec::default());
    }

    #[test]
    fn test_build_loader_no_name_no_dirs_returns_usable_loader() {
        // Falls back to SWE_EDGE_CONFIG_DIR or "config/" — either way, no panic.
        let result: Result<Sec, _> = DefaultConfigBuilder::new()
            .build_loader()
            .load_section("nonexistent_section_xyz");
        assert!(result.is_ok());
    }
}
