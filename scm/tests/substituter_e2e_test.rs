//! End-to-end tests for the substituter (env-var substitution in TOML values).
#![cfg(feature = "test-utils")]
#![allow(clippy::unwrap_used)]
#![allow(unsafe_code)]

use std::io::Write as _;
use tempfile::TempDir;

use swe_edge_configbuilder::{AllowAllPolicy, ConfigLoaderFactory};

fn write_toml(dir: &std::path::Path, content: &str) {
    let mut f = std::fs::File::create(dir.join("application.toml")).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct DbConfig {
    url: String,
}

/// @covers: substituter::Substituter
#[test]
fn test_substituter_replaces_env_var_placeholder_with_value() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[db]\nurl = \"postgresql://{{DB_HOST}}/mydb\"");

    // SAFETY: single-threaded test binary; no other thread reads or writes this var
    unsafe { std::env::set_var("DB_HOST", "localhost:5432") };
    let loader = ConfigLoaderFactory::create_loader_for_dir_with_substitution(
        dir.path(),
        Box::new(AllowAllPolicy),
    );
    let cfg: DbConfig = loader.load_section("db").unwrap();
    // SAFETY: cleanup — same invariant as above
    unsafe { std::env::remove_var("DB_HOST") };

    assert_eq!(cfg.url, "postgresql://localhost:5432/mydb");
}

/// @covers: substituter::Substituter
#[test]
fn test_substituter_returns_error_when_env_var_missing() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[db]\nurl = \"postgresql://{{UNDEFINED_VAR_XYZ}}/mydb\"",
    );

    // SAFETY: cleanup only — no other thread touches this var
    unsafe { std::env::remove_var("UNDEFINED_VAR_XYZ") };
    let loader = ConfigLoaderFactory::create_loader_for_dir_with_substitution(
        dir.path(),
        Box::new(AllowAllPolicy),
    );
    let result: Result<DbConfig, _> = loader.load_section("db");

    assert!(result.is_err(), "missing env var must return an error");
}
