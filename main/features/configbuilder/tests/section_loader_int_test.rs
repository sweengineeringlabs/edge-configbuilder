//! Integration tests for swe-edge-config section loading.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use swe_edge_configbuilder::ConfigLoaderFactory;

use std::io::Write as _;
use tempfile::TempDir;

use swe_edge_configbuilder::ConfigError;

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppSection {
    model: String,
    max_tokens: u32,
    enabled: bool,
}

fn write_toml(dir: &TempDir, name: &str, content: &str) {
    let path = dir.path().join(name);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::File::create(&path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}

// ── create_loader_for_dir ────────────────────────────────────────────────────

/// @covers: create_loader_for_dir
#[test]
fn test_load_section_from_reads_top_level_section() {
    let dir = TempDir::new().unwrap();
    write_toml(
        &dir,
        "application.toml",
        "[application.completion]\nmodel = \"gpt-4\"\nmax_tokens = 1024\nenabled = true",
    );
    let cfg: AppSection = ConfigLoaderFactory::create_loader_for_dir(dir.path())
        .load_section("application.completion")
        .unwrap();
    assert_eq!(cfg.model, "gpt-4");
    assert_eq!(cfg.max_tokens, 1024);
    assert!(cfg.enabled);
}

/// @covers: create_loader_for_dir
#[test]
fn test_load_section_from_returns_default_when_key_absent() {
    let dir = TempDir::new().unwrap();
    write_toml(&dir, "application.toml", "[other]\nvalue = 1");
    let cfg: AppSection = ConfigLoaderFactory::create_loader_for_dir(dir.path())
        .load_section("application.completion")
        .unwrap();
    assert_eq!(cfg, AppSection::default());
}

/// @covers: create_loader_for_dir
#[test]
fn test_load_section_from_returns_not_found_when_no_application_toml() {
    let dir = TempDir::new().unwrap();
    let result: Result<AppSection, _> =
        ConfigLoaderFactory::create_loader_for_dir(dir.path()).load_section("any_key");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for empty dir, got {result:?}"
    );
}

/// @covers: create_loader_for_dir
#[test]
fn test_load_section_from_rejects_invalid_toml() {
    let dir = TempDir::new().unwrap();
    write_toml(&dir, "application.toml", "not = [broken");
    let err = ConfigLoaderFactory::create_loader_for_dir(dir.path())
        .load_section::<AppSection>("key")
        .unwrap_err();
    assert!(matches!(err, ConfigError::Parse(_)));
}

/// @covers: create_loader_for_dir
#[test]
fn test_load_section_from_rejects_oversized_file() {
    let dir = TempDir::new().unwrap();
    let oversized = vec![b'#'; 1_048_577];
    std::fs::write(dir.path().join("application.toml"), &oversized).unwrap();
    let err = ConfigLoaderFactory::create_loader_for_dir(dir.path())
        .load_section::<AppSection>("key")
        .unwrap_err();
    assert!(matches!(err, ConfigError::Io(_)));
    assert!(err.to_string().contains("1 MiB"));
}

// ── create_loader_xdg ────────────────────────────────────────────────────────

/// @covers: create_loader_xdg
#[test]
fn test_load_section_xdg_unknown_app_returns_not_found() {
    let result: Result<AppSection, _> =
        ConfigLoaderFactory::create_loader_xdg("swe-edge-config-test-nonexistent-xyz")
            .unwrap()
            .load_section("application.completion");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for unknown XDG app, got {result:?}"
    );
}
