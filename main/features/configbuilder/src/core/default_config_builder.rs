//! `DefaultConfigBuilder` — default `ConfigBuilder` implementation.

use std::env;
use std::path::PathBuf;

use crate::api::traits::config_builder::ConfigBuilder;
use crate::api::traits::loader::Loader;
use crate::core::DefaultSectionLoader;

const CONFIG_DIR_ENV_VAR: &str = "SWE_EDGE_CONFIG_DIR";
const FALLBACK_CONFIG_DIR: &str = "config";

pub(crate) struct DefaultConfigBuilder {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) config_dirs: Vec<PathBuf>,
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
            return DefaultSectionLoader {
                config_dirs: self.config_dirs,
            };
        }

        if !self.name.is_empty() {
            let mut dirs: Vec<PathBuf> = Vec::new();
            let xdg_config_dirs =
                env::var("XDG_CONFIG_DIRS").unwrap_or_else(|_| "/etc/xdg".to_owned());
            for segment in xdg_config_dirs.split(':').rev() {
                if !segment.is_empty() {
                    dirs.push(PathBuf::from(segment).join(&self.name));
                }
            }
            if let Some(home) = dirs::config_dir() {
                dirs.push(home.join(&self.name));
            }
            if let Ok(v) = env::var(CONFIG_DIR_ENV_VAR) {
                dirs.push(PathBuf::from(v));
            }
            return DefaultSectionLoader { config_dirs: dirs };
        }

        let dir = env::var(CONFIG_DIR_ENV_VAR)
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(FALLBACK_CONFIG_DIR));
        DefaultSectionLoader {
            config_dirs: vec![dir],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    fn blank() -> DefaultConfigBuilder {
        DefaultConfigBuilder {
            name: String::new(),
            version: "0.1.0".to_string(),
            config_dirs: Vec::new(),
        }
    }

    #[test]
    fn test_blank_builder_has_empty_name_and_default_version() {
        let b = blank();
        assert_eq!(b.name(), "");
        assert_eq!(b.version(), "0.1.0");
    }

    #[test]
    fn test_with_name_sets_application_name() {
        let b = blank().with_name("swe-edge-config");
        assert_eq!(b.name(), "swe-edge-config");
    }

    #[test]
    fn test_with_version_sets_application_version() {
        let b = blank().with_version("2.0.0");
        assert_eq!(b.version(), "2.0.0");
    }

    #[test]
    fn test_with_config_dir_appends_to_config_dirs() {
        let b = blank().with_config_dir("/some/path");
        assert_eq!(b.config_dirs, vec![PathBuf::from("/some/path")]);
    }

    #[test]
    fn test_with_config_dir_multiple_calls_accumulate() {
        let b = blank().with_config_dir("/a").with_config_dir("/b");
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
        let sec: Sec = blank()
            .with_config_dir(dir.path())
            .build_loader()
            .load_section("svc")
            .unwrap();
        assert_eq!(sec.value, "ok");
    }

    #[test]
    fn test_build_loader_with_name_only_returns_usable_loader() {
        let sec: Sec = blank()
            .with_name("swe-edge-nonexistent-test-xyz")
            .build_loader()
            .load_section("any")
            .unwrap();
        assert_eq!(sec, Sec::default());
    }

    #[test]
    fn test_build_loader_no_name_no_dirs_returns_usable_loader() {
        let result: Result<Sec, _> = blank()
            .build_loader()
            .load_section("nonexistent_section_xyz");
        assert!(result.is_ok());
    }
}
