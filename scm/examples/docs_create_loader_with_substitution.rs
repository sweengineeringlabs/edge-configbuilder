//! `ConfigLoaderFactory::create_loader_with_substitution` — XDG-resolved loader
//! with `{{VAR_NAME}}` env var substitution enabled.
#![allow(unsafe_code)]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{ConfigLoaderFactory, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    greeting: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\ngreeting = \"{{DOCS_EXAMPLE_GREETING}}\"\n",
    )
    .expect("write application.toml");

    let policy =
        ConfigLoaderFactory::create_prefix_whitelist_policy(vec!["DOCS_EXAMPLE_".to_string()]);

    // SAFETY: single-threaded example process; no other thread touches these vars.
    unsafe {
        std::env::set_var("SWE_EDGE_CONFIG_DIR", dir.path());
        std::env::set_var("DOCS_EXAMPLE_GREETING", "hello from substitution");
    }
    let loader = ConfigLoaderFactory::create_loader_with_substitution(Box::new(policy))
        .expect("create_loader_with_substitution");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe {
        std::env::remove_var("SWE_EDGE_CONFIG_DIR");
        std::env::remove_var("DOCS_EXAMPLE_GREETING");
    }

    assert_eq!(cfg.greeting, "hello from substitution");
    println!("create_loader_with_substitution: greeting={}", cfg.greeting);
}
