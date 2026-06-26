//! Tests for SPI OptionalSection/ConfigSection surface.
//! @covers: spi::OptionalSection, spi::ConfigSection
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_configbuilder::{
    ConfigLoaderFactory, ConfigSection, FeatureState, OnError, OptionalSection,
};

use std::io::Write as _;
use tempfile::TempDir;

fn write_toml(dir: &std::path::Path, content: &str) {
    let mut f = std::fs::File::create(dir.join("application.toml")).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

// ── ConfigSection boundary ────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize, Default, PartialEq)]
struct SecretSection {
    token: String,
}

impl ConfigSection for SecretSection {
    fn section_name() -> &'static str {
        "secrets"
    }
}

#[test]
fn test_config_section_struct_load_section_absent_returns_default_not_error() {
    // Absence of a required section must yield Default, never expose internals.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[other]\nkey = \"value\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let result = SecretSection::load(&loader).unwrap();
    assert_eq!(result, SecretSection::default());
}

#[test]
fn test_config_section_struct_load_section_malformed_toml_returns_parse_error_not_panic() {
    // Malformed TOML must surface a typed error, not panic or expose a stack trace.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "not = [broken toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let err = SecretSection::load(&loader).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("parse") || msg.contains("TOML") || msg.contains("invalid"),
        "error message must not leak internal paths or stack state: {msg}"
    );
    assert!(
        !msg.contains("unwrap") && !msg.contains("panic"),
        "error must not expose implementation internals: {msg}"
    );
}

// ── OptionalSection boundary ──────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize)]
struct FeatureSection {
    host: String,
    port: u16,
}

impl OptionalSection for FeatureSection {
    fn section_name() -> &'static str {
        "feature_svc"
    }
}

#[test]
fn test_optional_section_struct_load_optional_absent_section_returns_disabled_not_error() {
    // Absent optional section must never propagate an error — only Disabled.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[unrelated]\nkey = \"x\"");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = FeatureSection::load_optional(&loader).unwrap();
    assert!(
        state.is_disabled(),
        "absent optional section must be Disabled, not an error"
    );
}

#[test]
fn test_optional_section_struct_load_optional_no_files_returns_disabled_not_error() {
    // No config files at all must yield Disabled, not a NotFound error.
    let dir = TempDir::new().unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = FeatureSection::load_optional(&loader).unwrap();
    assert!(state.is_disabled());
}

#[test]
fn test_optional_section_struct_load_optional_present_deserializes_correctly() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[feature_svc]\nhost = \"localhost\"\nport = 8080",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = FeatureSection::load_optional(&loader).unwrap();
    assert!(state.is_enabled());
    let cfg = state.into_option().unwrap();
    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 8080);
}

// ── validate_enabled isolation ────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize)]
struct StrictFeature {
    require_tls: bool,
    cert_path: Option<String>,
}

impl OptionalSection for StrictFeature {
    fn section_name() -> &'static str {
        "strict_feature"
    }

    fn on_error() -> OnError {
        OnError::Fail
    }

    fn validate_enabled(&self) -> Result<(), swe_edge_configbuilder::ConfigError> {
        if self.require_tls && self.cert_path.is_none() {
            return Err(swe_edge_configbuilder::ConfigError::Validation {
                section: Self::section_name().to_string(),
                reason: "cert_path is required when require_tls = true".to_string(),
            });
        }
        Ok(())
    }
}

#[test]
fn test_strict_feature_struct_validate_enabled_rejects_missing_cert_when_tls_required() {
    // validate_enabled must reject cross-field violations — this is the primary
    // security boundary the SPI exposes to downstream consumers.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[strict_feature]\nrequire_tls = true");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let err = StrictFeature::load_optional(&loader).unwrap_err();
    assert!(
        err.to_string().contains("cert_path"),
        "validation error must name the violated constraint: {err}"
    );
}

#[test]
fn test_strict_feature_struct_validate_enabled_accepts_valid_tls_config() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[strict_feature]\nrequire_tls = true\ncert_path = \"/etc/certs/server.pem\"",
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = StrictFeature::load_optional(&loader).unwrap();
    assert!(
        matches!(state, FeatureState::Enabled(_)),
        "valid TLS config must be Enabled"
    );
}

#[test]
fn test_strict_feature_struct_validate_enabled_accepts_no_tls_without_cert() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[strict_feature]\nrequire_tls = false");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let state = StrictFeature::load_optional(&loader).unwrap();
    assert!(
        matches!(state, FeatureState::Enabled(_)),
        "no-TLS config without cert must be Enabled"
    );
}
