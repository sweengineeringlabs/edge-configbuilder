//! Tests for OptionalSection trait — load_optional and validate_enabled.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use swe_edge_configbuilder::{ConfigError, ConfigLoaderFactory, FeatureStateOps as _, OptionalSection};
use tempfile::TempDir;

fn write_toml(dir: &std::path::Path, content: &str) {
    std::fs::write(dir.join("application.toml"), content).unwrap();
}

// ── test structs ──────────────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize, PartialEq)]
struct BrokerConfig {
    host: String,
    port: u16,
    #[serde(default)]
    tls_enabled: bool,
    cert_path: Option<String>,
}

impl OptionalSection for BrokerConfig {
    fn section_name() -> &'static str {
        "message_broker"
    }

    fn validate_enabled(&self) -> Result<(), ConfigError> {
        if self.tls_enabled && self.cert_path.is_none() {
            return Err(ConfigError::Validation {
                section: Self::section_name().to_string(),
                reason: "cert_path is required when tls_enabled = true".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize)]
struct SimpleFeature {
    name: String,
}

impl OptionalSection for SimpleFeature {
    fn section_name() -> &'static str {
        "simple_feature"
    }
    // validate_enabled: default Ok(()) — no extra constraints
}

// ── section absent ────────────────────────────────────────────────────────────

#[test]
fn test_optional_section_load_optional_absent_key_returns_disabled() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[other_section]\nvalue = \"x\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = BrokerConfig::load_optional(&loader).unwrap();
    assert!(
        state.is_disabled(),
        "absent section must return Disabled, not an error"
    );
}

#[test]
fn test_optional_section_load_optional_no_files_returns_disabled() {
    let dir = TempDir::new().unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = BrokerConfig::load_optional(&loader).unwrap();
    assert!(state.is_disabled());
}

// ── section present and valid ─────────────────────────────────────────────────

#[test]
fn test_optional_section_load_optional_present_valid_returns_enabled() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[message_broker]\nhost = \"mq.local\"\nport = 5672",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = BrokerConfig::load_optional(&loader).unwrap();
    assert!(state.is_enabled());
    let cfg = state.into_option().unwrap();
    assert_eq!(cfg.host, "mq.local");
    assert_eq!(cfg.port, 5672);
}

#[test]
fn test_optional_section_load_optional_tls_disabled_no_cert_passes_validation() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[message_broker]\nhost = \"mq.local\"\nport = 5672\ntls_enabled = false",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = BrokerConfig::load_optional(&loader).unwrap();
    assert!(state.is_enabled());
}

#[test]
fn test_optional_section_load_optional_tls_enabled_with_cert_passes_validation() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[message_broker]\nhost = \"mq.local\"\nport = 5672\ntls_enabled = true\ncert_path = \"/etc/certs/broker.pem\"",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = BrokerConfig::load_optional(&loader).unwrap();
    assert!(state.is_enabled());
    let cfg = state.into_option().unwrap();
    assert_eq!(cfg.cert_path.as_deref(), Some("/etc/certs/broker.pem"));
}

// ── validation failure ────────────────────────────────────────────────────────

#[test]
fn test_optional_section_load_optional_tls_enabled_missing_cert_returns_validation_error() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[message_broker]\nhost = \"mq.local\"\nport = 5672\ntls_enabled = true",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let err = BrokerConfig::load_optional(&loader).unwrap_err();
    assert!(
        matches!(err, ConfigError::Validation { .. }),
        "expected Validation error when tls_enabled = true but cert_path is absent, got {err:?}"
    );
    assert!(
        err.to_string().contains("cert_path"),
        "error message must mention the offending field"
    );
    assert!(
        err.to_string().contains("message_broker"),
        "error message must include the section name"
    );
}

#[test]
fn test_optional_section_load_optional_validate_not_called_when_section_absent() {
    // If validation were called on a disabled state the test would still pass
    // (validate_enabled returns Ok by default on SimpleFeature), but using
    // BrokerConfig — which panics when called with tls_enabled=true and no cert
    // — proves the path is skipped when Disabled.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[unrelated]\nkey = \"value\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    // No panic or error even though BrokerConfig's validate_enabled would reject
    // a tls_enabled=true section — it's never called here.
    let state = BrokerConfig::load_optional(&loader).unwrap();
    assert!(state.is_disabled());
}

// ── default validate_enabled ──────────────────────────────────────────────────

#[test]
fn test_optional_section_load_optional_default_validation_always_passes() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[simple_feature]\nname = \"demo\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = SimpleFeature::load_optional(&loader).unwrap();
    assert!(state.is_enabled());
    assert_eq!(state.into_option().unwrap().name, "demo");
}

// ── ConfigError::validation constructor ──────────────────────────────────────

#[test]
fn test_config_error_validation_constructor_formats_section_and_reason() {
    let err = ConfigError::Validation {
        section: "my_section".to_string(),
        reason: "host must not be empty".to_string(),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("my_section"),
        "error must include the section name"
    );
    assert!(
        msg.contains("host must not be empty"),
        "error must include the reason"
    );
}
