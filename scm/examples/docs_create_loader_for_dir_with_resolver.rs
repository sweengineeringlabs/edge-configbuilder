//! `ConfigLoaderFactory::create_loader_for_dir_with_resolver` — explicit
//! directory, with substitution values sourced from a custom `ValueResolver`.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{ConfigLoaderFactory, Loader as _, SubstitutionError, ValueResolver};

struct StaticResolver;
impl ValueResolver for StaticResolver {
    fn resolve(&self, var_name: &str, location: &str) -> Result<String, SubstitutionError> {
        match var_name {
            "APP_GREETING" => Ok("hello from a fixed dir resolver".to_string()),
            _ => Err(SubstitutionError::VariableNotFound {
                var_name: var_name.to_string(),
                location: location.to_string(),
            }),
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    greeting: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\ngreeting = \"{{APP_GREETING}}\"\n",
    )
    .expect("write application.toml");

    let policy = ConfigLoaderFactory::create_prefix_whitelist_policy(vec!["APP_".to_string()]);
    let loader = ConfigLoaderFactory::create_loader_for_dir_with_resolver(
        dir.path(),
        Box::new(policy),
        Box::new(StaticResolver),
    );
    let cfg: AppConfig = loader.load_section("app").expect("load app section");

    assert_eq!(cfg.greeting, "hello from a fixed dir resolver");
    println!(
        "create_loader_for_dir_with_resolver: greeting={}",
        cfg.greeting
    );
}
