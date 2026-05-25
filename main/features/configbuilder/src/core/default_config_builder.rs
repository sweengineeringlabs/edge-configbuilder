//! `DefaultConfigBuilder` — default `ConfigBuilder` implementation.

use std::env;
use std::path::{Path, PathBuf};

use crate::api::error::config_error::ConfigError;
use crate::api::traits::config_builder::ConfigBuilder;
use crate::api::traits::loader::Loader;
use crate::core::DefaultSectionLoader;

/// Reject paths that contain `..` components — guards against env-var traversal.
pub(crate) fn reject_traversal(path: &Path) -> Result<(), ConfigError> {
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(ConfigError::Io(format!(
            "{}: path traversal via '..' is not permitted",
            path.display()
        )));
    }
    Ok(())
}

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

    fn build_loader(self) -> Result<impl Loader, ConfigError> {
        if !self.config_dirs.is_empty() {
            let loader = DefaultSectionLoader {
                config_dirs: self.config_dirs,
            };
            loader.validate()?;
            return Ok(loader);
        }

        if !self.name.is_empty() {
            let mut dirs: Vec<PathBuf> = Vec::new();
            let xdg_config_dirs =
                env::var("XDG_CONFIG_DIRS").unwrap_or_else(|_| "/etc/xdg".to_owned());
            for segment in xdg_config_dirs.split(':').rev() {
                if !segment.is_empty() {
                    let seg_path = PathBuf::from(segment);
                    reject_traversal(&seg_path)?;
                    dirs.push(seg_path.join(&self.name));
                }
            }
            if let Some(home) = dirs::config_dir() {
                dirs.push(home.join(&self.name));
            }
            if let Ok(v) = env::var(CONFIG_DIR_ENV_VAR) {
                let p = PathBuf::from(&v);
                reject_traversal(&p)?;
                dirs.push(p);
            }
            let loader = DefaultSectionLoader { config_dirs: dirs };
            loader.validate()?;
            return Ok(loader);
        }

        let dir = match env::var(CONFIG_DIR_ENV_VAR) {
            Ok(v) => {
                let p = PathBuf::from(&v);
                reject_traversal(&p)?;
                p
            }
            Err(_) => PathBuf::from(FALLBACK_CONFIG_DIR),
        };
        let loader = DefaultSectionLoader {
            config_dirs: vec![dir],
        };
        loader.validate()?;
        Ok(loader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::error::config_error::ConfigError;
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
            .unwrap()
            .load_section("svc")
            .unwrap();
        assert_eq!(sec.value, "ok");
    }

    #[test]
    fn test_build_loader_with_unknown_name_returns_not_found() {
        let result: Result<Sec, _> = blank()
            .with_name("swe-edge-nonexistent-test-xyz")
            .build_loader()
            .unwrap()
            .load_section("any");
        assert!(
            matches!(result, Err(ConfigError::NotFound(_))),
            "expected NotFound for unknown app, got {result:?}"
        );
    }

    #[test]
    fn test_build_loader_no_name_no_dirs_returns_not_found() {
        let result: Result<Sec, _> = blank()
            .build_loader()
            .unwrap()
            .load_section("nonexistent_section_xyz");
        assert!(
            matches!(result, Err(ConfigError::NotFound(_))),
            "expected NotFound for absent section with no config, got {result:?}"
        );
    }

    #[test]
    fn test_reject_traversal_rejects_dotdot_path() {
        assert!(
            reject_traversal(std::path::Path::new("../../etc")).is_err(),
            "expected Io error for '..' path"
        );
    }

    #[test]
    fn test_reject_traversal_accepts_absolute_path() {
        assert!(
            reject_traversal(std::path::Path::new("/etc/xdg/myapp")).is_ok(),
            "expected ok for absolute path without '..'"
        );
    }
}
