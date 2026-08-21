//! End-to-end tests for pluggable `ValueResolver` — issue #13.
#![allow(clippy::unwrap_used)]
#![allow(unsafe_code)]

use std::collections::HashMap;
use std::io::Write as _;
use configbuilder::{ConfigLoaderFactory, Loader as _, SubstitutionError, ValueResolver};
use tempfile::TempDir;

fn write_toml(dir: &std::path::Path, content: &str) {
    let mut f = std::fs::File::create(dir.join("application.toml")).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct DbConfig {
    url: String,
}

/// A resolver backed by an in-memory map instead of the process environment.
struct MapValueResolver(HashMap<&'static str, &'static str>);

impl ValueResolver for MapValueResolver {
    fn resolve(&self, var_name: &str, location: &str) -> Result<String, SubstitutionError> {
        self.0.get(var_name).map(|v| v.to_string()).ok_or_else(|| {
            SubstitutionError::VariableNotFound {
                var_name: var_name.to_string(),
                location: location.to_string(),
            }
        })
    }
}

#[test]
fn test_custom_resolver_supplies_value_instead_of_env_var() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[db]\nurl = \"postgresql://{{APP_DB_HOST}}/mydb\"",
    );

    // An env var of the same name holds a DIFFERENT value — if this test passes,
    // it proves the custom resolver was actually invoked, not a silent env fallback.
    // SAFETY: single-threaded test binary; no other thread reads or writes this var.
    unsafe { std::env::set_var("APP_DB_HOST", "env-value-must-not-be-used") };

    let mut map = HashMap::new();
    map.insert("APP_DB_HOST", "resolver-value.internal:5432");
    let resolver = MapValueResolver(map);

    let loader = ConfigLoaderFactory::create_loader_for_dir_with_resolver(
        dir.path(),
        Box::new(ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
            "APP_".to_string(),
        ])),
        Box::new(resolver),
    );
    let cfg: DbConfig = loader.load_section("db").unwrap();

    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("APP_DB_HOST") };

    assert_eq!(cfg.url, "postgresql://resolver-value.internal:5432/mydb");
}

#[test]
fn test_custom_resolver_missing_value_returns_error() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[db]\nurl = \"postgresql://{{APP_UNKNOWN}}/mydb\"",
    );

    let resolver = MapValueResolver(HashMap::new());
    let loader = ConfigLoaderFactory::create_loader_for_dir_with_resolver(
        dir.path(),
        Box::new(ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
            "APP_".to_string(),
        ])),
        Box::new(resolver),
    );
    let result: Result<DbConfig, _> = loader.load_section("db");

    assert!(
        result.is_err(),
        "unresolvable variable from a custom resolver must return an error"
    );
}

#[test]
fn test_custom_resolver_still_gated_by_substitution_policy() {
    let dir = TempDir::new().unwrap();
    write_toml(
        dir.path(),
        "[db]\nurl = \"postgresql://{{FORBIDDEN_HOST}}/mydb\"",
    );

    let mut map = HashMap::new();
    map.insert("FORBIDDEN_HOST", "should-never-be-reached");
    let resolver = MapValueResolver(map);

    let loader = ConfigLoaderFactory::create_loader_for_dir_with_resolver(
        dir.path(),
        // Whitelist only allows "APP_" — "FORBIDDEN_HOST" must be rejected by the
        // policy before the resolver is ever consulted.
        Box::new(ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
            "APP_".to_string(),
        ])),
        Box::new(resolver),
    );
    let result: Result<DbConfig, _> = loader.load_section("db");

    assert!(
        result.is_err(),
        "policy-rejected variable name must error even though the resolver has a value for it"
    );
}
