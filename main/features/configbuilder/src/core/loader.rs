//! `DefaultSectionLoader` — layered TOML section extractor with optional substitution.

use std::path::PathBuf;

const MAX_CONFIG_FILE_BYTES: u64 = 1_048_576;
const NOT_A_DIR_MSG: &str = "config path exists but is not a directory";
use crate::api::feature::types::feature_metadata::FeatureMetadata;
use crate::api::feature::types::feature_record::FeatureRecord;
use crate::api::feature::types::feature_state::FeatureState;
use crate::api::feature::types::loaded_feature::LoadedFeature;
use crate::api::feature::types::override_source::OverrideSource;
use crate::api::loader::errors::config_error::ConfigError;
use crate::api::traits::feature_loader::FeatureLoader;
use crate::api::traits::loader::Loader;
use crate::api::traits::substitution_policy::SubstitutionPolicy;
use crate::core::Substituter;

/// Loads an arbitrary TOML section from a layered chain of config directories.
///
/// Each directory's `application.toml` is merged in order; later entries win.
/// Merging is **recursive**: when both the base and overlay contain a TOML table at
/// the same key path, their sub-keys are merged rather than the overlay replacing
/// the entire table. Arrays and scalars are always replaced by the overlay value.
///
/// Optionally applies environment variable substitution (`{{VAR_NAME}}` syntax)
/// if created with a substitution policy.
pub(crate) struct DefaultSectionLoader {
    pub(crate) config_dirs: Vec<PathBuf>,
    pub(crate) substitution_policy: Option<Box<dyn SubstitutionPolicy>>,
}

impl DefaultSectionLoader {
    fn merge_toml(base: toml::Value, overlay: toml::Value) -> toml::Value {
        match (base, overlay) {
            (toml::Value::Table(mut b), toml::Value::Table(o)) => {
                for (k, v) in o {
                    let merged = match b.remove(&k) {
                        Some(base_v) => Self::merge_toml(base_v, v),
                        None => v,
                    };
                    b.insert(k, merged);
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
        let mut any_file_found = false;
        let mut merged = toml::Value::Table(toml::map::Map::new());

        for dir in &self.config_dirs {
            let path = dir.join("application.toml");
            if !path.exists() {
                continue;
            }
            any_file_found = true;
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

            let text = if let Some(ref policy) = self.substitution_policy {
                let substituter =
                    Substituter::new(policy.as_ref(), format!("{}:{}", path.display(), key));
                substituter
                    .substitute(&text)
                    .map_err(|e| ConfigError::Io(e.to_string()))?
            } else {
                text
            };

            let val: toml::Value =
                toml::from_str(&text).map_err(|e| ConfigError::Parse(e.to_string()))?;
            if let Some(section) = Self::extract_dotted(&val, key) {
                merged = Self::merge_toml(merged, section);
            }
        }

        if matches!(merged, toml::Value::Table(ref t) if t.is_empty()) {
            if !any_file_found {
                return Err(ConfigError::NotFound(format!(
                    "no application.toml found in any configured directory for section '{key}'"
                )));
            }
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

impl DefaultSectionLoader {
    /// Convert a TOML section key to its `SWE_EDGE_FEATURE_*` env var name.
    fn feature_env_var_name(key: &str) -> String {
        let suffix = key.to_uppercase().replace('.', "_");
        format!("SWE_EDGE_FEATURE_{suffix}")
    }

    /// Parse an env var value as a boolean toggle.
    ///
    /// Returns `Some(true/false)` for recognised values, `None` for the empty
    /// string (var not set), and `Err` for unrecognised non-empty values.
    fn parse_feature_env_var(val: &str, var_name: &str) -> Result<Option<bool>, ConfigError> {
        match val.to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Ok(Some(true)),
            "false" | "0" | "no" | "off" => Ok(Some(false)),
            _ => Err(ConfigError::Io(format!(
                "invalid value for env var {var_name}: '{val}'; \
                 accepted values: true/false, 1/0, yes/no, on/off"
            ))),
        }
    }

    /// Read every `application.toml` across all config dirs, apply substitution,
    /// and merge the section at `key`.  Returns `(section_found, merged_value)`.
    fn scan_and_merge(&self, key: &str) -> Result<(bool, toml::Value), ConfigError> {
        let mut merged = toml::Value::Table(toml::map::Map::new());
        let mut section_found = false;

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

            let text = if let Some(ref policy) = self.substitution_policy {
                let substituter =
                    Substituter::new(policy.as_ref(), format!("{}:{}", path.display(), key));
                substituter
                    .substitute(&text)
                    .map_err(|e| ConfigError::Io(e.to_string()))?
            } else {
                text
            };

            let val: toml::Value =
                toml::from_str(&text).map_err(|e| ConfigError::Parse(e.to_string()))?;
            if let Some(section) = Self::extract_dotted(&val, key) {
                section_found = true;
                merged = Self::merge_toml(merged, section);
            }
        }

        Ok((section_found, merged))
    }
}

impl FeatureLoader for DefaultSectionLoader {
    fn load_feature<T>(&self, key: &str) -> Result<LoadedFeature<T>, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        let var_name = Self::feature_env_var_name(key);

        // ── 1. Env-var override (highest precedence) ─────────────────────────
        let env_force: Option<bool> = match std::env::var(&var_name) {
            Err(_) => None, // var not set
            Ok(ref val) => Self::parse_feature_env_var(val, &var_name)?,
        };

        if env_force == Some(false) {
            return Ok(LoadedFeature {
                state: FeatureState::Disabled,
                record: FeatureRecord {
                    section_name: key.to_owned(),
                    enabled: false,
                    override_source: Some(OverrideSource::EnvVar {
                        var_name,
                        value: std::env::var(Self::feature_env_var_name(key)).unwrap_or_default(),
                    }),
                    requires: &[],
                    metadata: FeatureMetadata::default(),
                },
            });
        }

        // ── 2. TOML scan ─────────────────────────────────────────────────────
        let (section_found, merged) = self.scan_and_merge(key)?;

        if !section_found {
            if env_force == Some(true) {
                // Operator explicitly requested enable but section is absent.
                return Err(ConfigError::NotFound(format!(
                    "env var {var_name}=true but section '{key}' \
                     is absent from all config files; add [{key}] to application.toml"
                )));
            }
            return Ok(LoadedFeature {
                state: FeatureState::Disabled,
                record: FeatureRecord {
                    section_name: key.to_owned(),
                    enabled: false,
                    override_source: None,
                    requires: &[],
                    metadata: FeatureMetadata::default(),
                },
            });
        }

        // ── 3. Explicit `enabled = false` in TOML (skipped when env forces on) ─
        if env_force != Some(true) {
            if let Some(toml::Value::Boolean(false)) = merged.get("enabled") {
                return Ok(LoadedFeature {
                    state: FeatureState::Disabled,
                    record: FeatureRecord {
                        section_name: key.to_owned(),
                        enabled: false,
                        override_source: Some(OverrideSource::ExplicitTomlFlag),
                        requires: &[],
                        metadata: FeatureMetadata::default(),
                    },
                });
            }
        }

        // ── 4. Deserialise ───────────────────────────────────────────────────
        let value: T = merged
            .try_into()
            .map_err(|e: toml::de::Error| ConfigError::Parse(e.to_string()))?;

        let override_source = env_force.map(|_| OverrideSource::EnvVar {
            var_name,
            value: "true".into(),
        });

        Ok(LoadedFeature {
            state: FeatureState::Enabled(value),
            record: FeatureRecord {
                section_name: key.to_owned(),
                enabled: true,
                override_source,
                requires: &[],
                metadata: FeatureMetadata::default(),
            },
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, unsafe_code)]
mod tests {
    use super::*;
    use crate::api::feature::types::feature_state::FeatureState;
    use crate::api::loader::errors::config_error::ConfigError;
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
            substitution_policy: None,
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
    fn test_load_section_returns_not_found_when_no_application_toml() {
        let dir = TempDir::new().unwrap();
        let result: Result<Sec, _> = loader_in(dir.path()).load_section("nonexistent");
        assert!(
            matches!(result, Err(ConfigError::NotFound(_))),
            "expected NotFound when no application.toml exists, got {result:?}"
        );
    }

    #[test]
    fn test_load_section_returns_default_when_section_absent_from_existing_toml() {
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[other_section]\nvalue = \"x\"",
        );
        let sec: Sec = loader_in(dir.path()).load_section("nonexistent").unwrap();
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
            substitution_policy: None,
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
            substitution_policy: None,
        };
        let sec: Sec = loader.load_section("s").unwrap();
        assert_eq!(sec.value, "hi");
        assert_eq!(sec.count, 9);
    }

    #[derive(Debug, Default, serde::Deserialize, PartialEq)]
    #[serde(default)]
    struct Server {
        host: String,
        tls: Tls,
    }

    #[derive(Debug, Default, serde::Deserialize, PartialEq)]
    #[serde(default)]
    struct Tls {
        cert: String,
        key: String,
    }

    #[test]
    fn test_load_section_deep_merges_nested_tables_across_dirs() {
        // Regression: shallow merge replaced the entire `tls` subtable, dropping
        // any key in the base that was absent from the overlay.
        let low = TempDir::new().unwrap();
        let high = TempDir::new().unwrap();
        write_toml(
            low.path(),
            "application.toml",
            "[s]\nhost = \"localhost\"\n\n[s.tls]\ncert = \"old.pem\"\nkey = \"key.pem\"",
        );
        write_toml(
            high.path(),
            "application.toml",
            "[s.tls]\ncert = \"new.pem\"",
        );
        let loader = DefaultSectionLoader {
            config_dirs: vec![low.path().to_path_buf(), high.path().to_path_buf()],
            substitution_policy: None,
        };
        let srv: Server = loader.load_section("s").unwrap();
        assert_eq!(
            srv.host, "localhost",
            "host must survive overlay of sibling subtable"
        );
        assert_eq!(
            srv.tls.cert, "new.pem",
            "cert must be overridden by high-priority dir"
        );
        assert_eq!(
            srv.tls.key, "key.pem",
            "key must not be lost when only cert is overridden"
        );
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

    // ── load_optional_section ────────────────────────────────────────────────

    #[test]
    fn test_load_optional_section_present_key_returns_enabled() {
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[my_feature]\nvalue = \"on\"\ncount = 3",
        );
        let state: FeatureState<Sec> = loader_in(dir.path())
            .load_optional_section("my_feature")
            .unwrap();
        assert!(state.is_enabled(), "expected Enabled when key is present");
        let sec = state.into_option().unwrap();
        assert_eq!(sec.value, "on");
        assert_eq!(sec.count, 3);
    }

    #[test]
    fn test_load_optional_section_absent_key_in_existing_file_returns_disabled() {
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[other_section]\nvalue = \"x\"",
        );
        let state: FeatureState<Sec> = loader_in(dir.path())
            .load_optional_section("my_feature")
            .unwrap();
        assert!(
            state.is_disabled(),
            "expected Disabled when key absent from existing file"
        );
    }

    #[test]
    fn test_load_optional_section_no_files_returns_disabled() {
        let dir = TempDir::new().unwrap();
        let state: FeatureState<Sec> = loader_in(dir.path())
            .load_optional_section("my_feature")
            .unwrap();
        assert!(
            state.is_disabled(),
            "expected Disabled when no config files exist"
        );
    }

    #[test]
    fn test_load_optional_section_malformed_toml_returns_parse_error() {
        let dir = TempDir::new().unwrap();
        write_toml(dir.path(), "application.toml", "not = [broken toml");
        let err = loader_in(dir.path())
            .load_optional_section::<Sec>("my_feature")
            .unwrap_err();
        assert!(
            matches!(err, ConfigError::Parse(_)),
            "expected Parse error for malformed TOML, got {err:?}"
        );
    }

    #[test]
    fn test_load_optional_section_missing_required_field_returns_parse_error() {
        #[derive(Debug, serde::Deserialize)]
        struct Strict {
            required: String, // no Default, no Option — must be present
        }
        let dir = TempDir::new().unwrap();
        // Section present but `required` field is absent
        write_toml(dir.path(), "application.toml", "[feat]\nother = \"x\"");
        let err = loader_in(dir.path())
            .load_optional_section::<Strict>("feat")
            .unwrap_err();
        assert!(
            matches!(err, ConfigError::Parse(_)),
            "expected Parse for missing required field, got {err:?}"
        );
    }

    #[test]
    fn test_load_optional_section_multi_dir_merge_returns_enabled() {
        let low = TempDir::new().unwrap();
        let high = TempDir::new().unwrap();
        write_toml(low.path(), "application.toml", "[feat]\ncount = 1");
        write_toml(high.path(), "application.toml", "[feat]\nvalue = \"hi\"");
        let loader = DefaultSectionLoader {
            config_dirs: vec![low.path().to_path_buf(), high.path().to_path_buf()],
            substitution_policy: None,
        };
        let state: FeatureState<Sec> = loader.load_optional_section("feat").unwrap();
        assert!(state.is_enabled());
        let sec = state.into_option().unwrap();
        assert_eq!(sec.value, "hi");
        assert_eq!(sec.count, 1);
    }

    #[test]
    fn test_load_optional_section_multi_dir_both_absent_returns_disabled() {
        let low = TempDir::new().unwrap();
        let high = TempDir::new().unwrap();
        write_toml(low.path(), "application.toml", "[other]\nvalue = \"x\"");
        write_toml(
            high.path(),
            "application.toml",
            "[also_other]\nvalue = \"y\"",
        );
        let loader = DefaultSectionLoader {
            config_dirs: vec![low.path().to_path_buf(), high.path().to_path_buf()],
            substitution_policy: None,
        };
        let state: FeatureState<Sec> = loader.load_optional_section("feat").unwrap();
        assert!(state.is_disabled());
    }

    // ── load_feature ─────────────────────────────────────────────────────────

    // Env-var tests mutate process state — serialize them.
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn test_load_feature_section_present_returns_enabled_record() {
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[feat]\nvalue = \"on\"\ncount = 5",
        );
        let loaded = loader_in(dir.path()).load_feature::<Sec>("feat").unwrap();
        assert!(loaded.state.is_enabled());
        assert!(loaded.record.enabled);
        assert!(loaded.record.override_source.is_none());
        assert_eq!(loaded.record.section_name, "feat");
    }

    #[test]
    fn test_load_feature_section_absent_returns_disabled_record() {
        let dir = TempDir::new().unwrap();
        write_toml(dir.path(), "application.toml", "[other]\nvalue = \"x\"");
        let loaded = loader_in(dir.path()).load_feature::<Sec>("feat").unwrap();
        assert!(loaded.state.is_disabled());
        assert!(!loaded.record.enabled);
        assert!(loaded.record.override_source.is_none());
    }

    #[test]
    fn test_load_feature_enabled_false_in_toml_returns_disabled_with_explicit_flag() {
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[feat]\nenabled = false\nvalue = \"x\"",
        );
        let loaded = loader_in(dir.path()).load_feature::<Sec>("feat").unwrap();
        assert!(loaded.state.is_disabled());
        assert!(!loaded.record.enabled);
        assert!(
            matches!(
                loaded.record.override_source,
                Some(crate::api::feature::types::override_source::OverrideSource::ExplicitTomlFlag)
            ),
            "expected ExplicitTomlFlag override source"
        );
    }

    #[test]
    fn test_load_feature_env_var_false_disables_present_section() {
        let _g = ENV_LOCK.lock().unwrap();
        let var = "SWE_EDGE_FEATURE_FEAT_LF_ENV_OFF";
        // SAFETY: test-only, serialized by ENV_LOCK
        unsafe { std::env::set_var(var, "false") };
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[feat_lf_env_off]\nvalue = \"on\"",
        );
        let result = loader_in(dir.path()).load_feature::<Sec>("feat_lf_env_off");
        // SAFETY: cleanup
        unsafe { std::env::remove_var(var) };
        let loaded = result.unwrap();
        assert!(loaded.state.is_disabled());
        assert!(!loaded.record.enabled);
        assert!(
            matches!(
                loaded.record.override_source,
                Some(crate::api::feature::types::override_source::OverrideSource::EnvVar { .. })
            ),
            "expected EnvVar override source"
        );
    }

    #[test]
    fn test_load_feature_env_var_true_enables_present_section() {
        let _g = ENV_LOCK.lock().unwrap();
        let var = "SWE_EDGE_FEATURE_FEAT_LF_ENV_ON";
        // SAFETY: test-only, serialized by ENV_LOCK
        unsafe { std::env::set_var(var, "true") };
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[feat_lf_env_on]\nvalue = \"on\"",
        );
        let result = loader_in(dir.path()).load_feature::<Sec>("feat_lf_env_on");
        // SAFETY: cleanup
        unsafe { std::env::remove_var(var) };
        let loaded = result.unwrap();
        assert!(loaded.state.is_enabled());
        assert!(loaded.record.enabled);
        assert!(
            matches!(
                loaded.record.override_source,
                Some(crate::api::feature::types::override_source::OverrideSource::EnvVar { .. })
            ),
            "expected EnvVar override source"
        );
    }

    #[test]
    fn test_load_feature_env_var_true_overrides_enabled_false_in_toml() {
        let _g = ENV_LOCK.lock().unwrap();
        let var = "SWE_EDGE_FEATURE_FEAT_LF_FORCE_ON";
        // SAFETY: test-only, serialized by ENV_LOCK
        unsafe { std::env::set_var(var, "1") };
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[feat_lf_force_on]\nenabled = false\nvalue = \"x\"",
        );
        let result = loader_in(dir.path()).load_feature::<Sec>("feat_lf_force_on");
        // SAFETY: cleanup
        unsafe { std::env::remove_var(var) };
        let loaded = result.unwrap();
        assert!(
            loaded.state.is_enabled(),
            "env var=true must override enabled=false in TOML"
        );
    }

    #[test]
    fn test_load_feature_env_var_true_section_absent_returns_not_found() {
        let _g = ENV_LOCK.lock().unwrap();
        let var = "SWE_EDGE_FEATURE_FEAT_LF_ABSENT";
        // SAFETY: test-only, serialized by ENV_LOCK
        unsafe { std::env::set_var(var, "true") };
        let dir = TempDir::new().unwrap();
        write_toml(dir.path(), "application.toml", "[other]\nvalue = \"x\"");
        let result = loader_in(dir.path()).load_feature::<Sec>("feat_lf_absent");
        // SAFETY: cleanup
        unsafe { std::env::remove_var(var) };
        assert!(
            matches!(result, Err(ConfigError::NotFound(_))),
            "env var=true + absent section must be NotFound, got {result:?}"
        );
    }

    #[test]
    fn test_load_feature_invalid_env_var_value_returns_io_error() {
        let _g = ENV_LOCK.lock().unwrap();
        let var = "SWE_EDGE_FEATURE_FEAT_LF_INVALID";
        // SAFETY: test-only, serialized by ENV_LOCK
        unsafe { std::env::set_var(var, "maybe") };
        let dir = TempDir::new().unwrap();
        write_toml(
            dir.path(),
            "application.toml",
            "[feat_lf_invalid]\nvalue = \"x\"",
        );
        let result = loader_in(dir.path()).load_feature::<Sec>("feat_lf_invalid");
        // SAFETY: cleanup
        unsafe { std::env::remove_var(var) };
        assert!(
            matches!(result, Err(ConfigError::Io(_))),
            "invalid env var value must return Io error, got {result:?}"
        );
    }

    #[test]
    fn test_validate_accepts_nonexistent_dir() {
        let loader = DefaultSectionLoader {
            config_dirs: vec![PathBuf::from("/nonexistent/swe-edge-test-xyz")],
            substitution_policy: None,
        };
        assert!(loader.validate().is_ok());
    }

    #[test]
    fn test_validate_accepts_existing_dir() {
        let dir = TempDir::new().unwrap();
        let loader = DefaultSectionLoader {
            config_dirs: vec![dir.path().to_path_buf()],
            substitution_policy: None,
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
            substitution_policy: None,
        };
        let err = loader.validate().unwrap_err();
        assert!(matches!(err, ConfigError::Io(_)));
        assert!(err.to_string().contains("not a directory"));
    }
}
