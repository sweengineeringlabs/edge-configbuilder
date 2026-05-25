//! Tests for the public section-loading operations on `Loader`.

use std::io::Write as _;
use swe_edge_configbuilder::{
    create_loader, create_loader_for_dir, create_loader_xdg, ConfigError, Loader as _,
};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct Cfg {
    value: String,
}

/// @covers: create_loader
#[test]
fn test_load_section_returns_not_found_for_absent_key() {
    let result: Result<Cfg, _> = create_loader()
        .unwrap()
        .load_section("nonexistent_section_xyz");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for absent key, got {result:?}"
    );
}

/// @covers: create_loader_for_dir
#[test]
fn test_load_section_from_returns_not_found_when_no_toml() {
    let dir = tempfile::tempdir().unwrap();
    let result: Result<Cfg, _> = create_loader_for_dir(dir.path()).load_section("nonexistent");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for empty dir, got {result:?}"
    );
}

/// @covers: create_loader_for_dir
#[test]
fn test_load_section_from_reads_written_section() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    writeln!(f, "[my_section]\nvalue = \"found\"").unwrap();
    let cfg: Cfg = create_loader_for_dir(dir.path())
        .load_section("my_section")
        .unwrap();
    assert_eq!(cfg.value, "found");
}

/// @covers: create_loader_xdg
#[test]
fn test_load_section_xdg_returns_not_found_for_unknown_app() {
    let result: Result<Cfg, _> = create_loader_xdg("swe-edge-test-nonexistent-xyz")
        .unwrap()
        .load_section("any_section");
    assert!(
        matches!(result, Err(ConfigError::NotFound(_))),
        "expected NotFound for unknown XDG app, got {result:?}"
    );
}
