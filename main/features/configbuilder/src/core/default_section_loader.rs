//! `DefaultSectionLoader` — layered TOML section extractor.

use std::path::PathBuf;

use crate::api::default_section_loader::MAX_CONFIG_FILE_BYTES;
use crate::api::default_validator::NOT_A_DIR_MSG;
use crate::api::error::config_error::ConfigError;
use crate::api::traits::loader::Loader;

/// Loads an arbitrary TOML section from a layered chain of config directories.
///
/// Each directory's `application.toml` is merged in order; later entries win.
/// Construct via [`DefaultSectionLoader::new`], [`DefaultSectionLoader::with_dir`], or
/// [`DefaultSectionLoader::xdg`].
pub(crate) struct DefaultSectionLoader {
    pub(crate) config_dirs: Vec<PathBuf>,
}

impl DefaultSectionLoader {
    fn merge_toml(base: toml::Value, overlay: toml::Value) -> toml::Value {
        match (base, overlay) {
            (toml::Value::Table(mut b), toml::Value::Table(o)) => {
                for (k, v) in o {
                    b.insert(k, v);
                }
                toml::Value::Table(b)
            }
            (_, o) => o,
        }
    }

    fn extract_dotted(val: &toml::Value, key: &str) -> Option<toml::Value> {
        let mut current = val;
        for part in key.split('.') {
            current = current.get(part)?;
        }
        Some(current.clone())
    }
}

impl Loader for DefaultSectionLoader {
    fn load_section<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let mut merged = toml::Value::Table(toml::map::Map::new());

        for dir in &self.config_dirs {
            let path = dir.join("application.toml");
            if !path.exists() {
                continue;
            }
            let meta = std::fs::metadata(&path)
                .map_err(|e| ConfigError::Io(format!("{}: {e}", path.display())))?;
            if meta.len() > MAX_CONFIG_FILE_BYTES {
                return Err(ConfigError::Io(format!(
                    "{}: config file exceeds the 1 MiB limit ({} bytes)",
                    path.display(),
                    meta.len(),
                )));
            }
            let text = std::fs::read_to_string(&path)
                .map_err(|e| ConfigError::Io(format!("{}: {e}", path.display())))?;
            let val: toml::Value =
                toml::from_str(&text).map_err(|e| ConfigError::Parse(e.to_string()))?;
            if let Some(section) = Self::extract_dotted(&val, key) {
                merged = Self::merge_toml(merged, section);
            }
        }

        if matches!(merged, toml::Value::Table(ref t) if t.is_empty()) {
            return Ok(T::default());
        }

        merged
            .try_into()
            .map_err(|e: toml::de::Error| ConfigError::Parse(e.to_string()))
    }

    fn validate(&self) -> Result<(), ConfigError> {
        for dir in &self.config_dirs {
            if dir.exists() && !dir.is_dir() {
                return Err(ConfigError::Io(format!(
                    "{}: {NOT_A_DIR_MSG}",
                    dir.display()
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;
    use std::path::Path;
    use tempfile::TempDir;

    #[derive(Debug, Default, serde::Deserialize, PartialEq)]
    #[serde(default)]
    struct Sec {
        value: String,
        count: u32,
    }

    fn loader_in(dir: &Path) -> DefaultSectionLoader {
        DefaultSectionLoader {
            config_dirs: vec![dir.to_path_buf()],
        }
    }

    fn write_toml(dir: &Path, name: &str, content: &str) {
        let path = dir.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::File::create(&path)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
    }

    #[test]
    fn test_load_section_reads_top_level_key() {
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[my_section]\nvalue = \"hello\"\ncount = 7",
        );
        let sec: Sec = loader_in(dir.path()).load_section("my_section").unwrap();
        assert_eq!(sec.value, "hello");
        assert_eq!(sec.count, 7);
    }

    #[test]
    fn test_load_section_returns_default_when_key_absent() {
        let dir = TempDir::new().unwrap();
        let sec: Sec = loader_in(dir.path()).load_section("nonexistent").unwrap();
        assert_eq!(sec, Sec::default());
    }

    #[test]
    fn test_load_section_returns_default_when_no_application_toml() {
        let dir = TempDir::new().unwrap();
        let sec: Sec = loader_in(dir.path()).load_section("any").unwrap();
        assert_eq!(sec, Sec::default());
    }

    #[test]
    fn test_load_section_supports_dotted_key_path() {
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[outer.inner]\nvalue = \"deep\"\ncount = 3",
        );
        let sec: Sec = loader_in(dir.path()).load_section("outer.inner").unwrap();
        assert_eq!(sec.value, "deep");
        assert_eq!(sec.count, 3);
    }

    #[test]
    fn test_load_section_later_dir_wins_over_earlier() {
        let low = TempDir::new().unwrap();
        let high = TempDir::new().unwrap();
        write_toml(low.path(), "application.toml", "[s]\nvalue = \"low\"");
        write_toml(high.path(), "application.toml", "[s]\nvalue = \"high\"");
        let loader = DefaultSectionLoader {
            config_dirs: vec![low.path().to_path_buf(), high.path().to_path_buf()],
        };
        let sec: Sec = loader.load_section("s").unwrap();
        assert_eq!(sec.value, "high");
    }

    #[test]
    fn test_load_section_earlier_dir_fills_unset_fields() {
        let low = TempDir::new().unwrap();
        let high = TempDir::new().unwrap();
        write_toml(low.path(), "application.toml", "[s]\ncount = 9");
        write_toml(high.path(), "application.toml", "[s]\nvalue = \"hi\"");
        let loader = DefaultSectionLoader {
            config_dirs: vec![low.path().to_path_buf(), high.path().to_path_buf()],
        };
        let sec: Sec = loader.load_section("s").unwrap();
        assert_eq!(sec.value, "hi");
        assert_eq!(sec.count, 9);
    }

    #[test]
    fn test_load_section_rejects_oversized_application_toml() {
        let dir = TempDir::new().unwrap();
        let oversized = vec![b'#'; (MAX_CONFIG_FILE_BYTES + 1) as usize];
        std::fs::write(dir.path().join("application.toml"), &oversized).unwrap();
        let err = loader_in(dir.path()).load_section::<Sec>("s").unwrap_err();
        assert!(matches!(err, ConfigError::Io(_)));
        assert!(err.to_string().contains("1 MiB"));
    }

    #[test]
    fn test_load_section_rejects_invalid_toml() {
        let dir = TempDir::new().unwrap();
        write_toml(dir.path(), "application.toml", "not = [broken toml");
        let err = loader_in(dir.path()).load_section::<Sec>("s").unwrap_err();
        assert!(matches!(err, ConfigError::Parse(_)));
    }

    #[test]
    fn test_validate_accepts_nonexistent_dir() {
        let loader = DefaultSectionLoader {
            config_dirs: vec![PathBuf::from("/nonexistent/swe-edge-test-xyz")],
        };
        assert!(loader.validate().is_ok());
    }

    #[test]
    fn test_validate_accepts_existing_dir() {
        let dir = TempDir::new().unwrap();
        let loader = DefaultSectionLoader {
            config_dirs: vec![dir.path().to_path_buf()],
        };
        assert!(loader.validate().is_ok());
    }

    #[test]
    fn test_validate_rejects_file_path() {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("not_a_dir.toml");
        std::fs::write(&file, b"").unwrap();
        let loader = DefaultSectionLoader {
            config_dirs: vec![file.clone()],
        };
        let err = loader.validate().unwrap_err();
        assert!(matches!(err, ConfigError::Io(_)));
        assert!(err.to_string().contains("not a directory"));
    }
}
