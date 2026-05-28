//! Tests for the LoaderOps internal trait via SectionLoaderImpl.
#![allow(clippy::unwrap_used)]

use std::io::Write as _;
use swe_edge_configbuilder::create_loader_for_dir;
use tempfile::TempDir;

fn dir_with(content: &str) -> TempDir {
    let dir = TempDir::new().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    dir
}

/// @covers: loader_ops::LoaderOps::load_section_value
#[test]
fn test_loader_ops_load_section_value_present_key_returns_value() {
    #[derive(serde::Deserialize, Default, PartialEq, Debug)]
    #[serde(default)]
    struct S {
        x: i32,
    }
    let dir = dir_with("[s]\nx = 42");
    let cfg: S = create_loader_for_dir(dir.path()).load_section("s").unwrap();
    assert_eq!(cfg.x, 42);
}

/// @covers: loader_ops::LoaderOps::validate_dirs
#[test]
fn test_loader_ops_validate_dirs_valid_dir_returns_ok() {
    let dir = TempDir::new().unwrap();
    assert!(create_loader_for_dir(dir.path()).validate().is_ok());
}
