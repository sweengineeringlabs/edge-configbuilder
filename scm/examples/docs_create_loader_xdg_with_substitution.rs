//! `ConfigLoaderFactory::create_loader_xdg_with_substitution` — XDG-resolved
//! loader for a named app, with `{{VAR_NAME}}` env var substitution enabled.
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
        "[app]\ngreeting = \"{{DOCS_EXAMPLE_XDG_GREETING}}\"\n",
    )
    .expect("write application.toml");

    let policy =
        ConfigLoaderFactory::create_prefix_whitelist_policy(vec!["DOCS_EXAMPLE_XDG_".to_string()]);

    // SAFETY: single-threaded example process; no other thread touches these vars.
    unsafe {
        std::env::set_var("SWE_EDGE_CONFIG_DIR", dir.path());
        std::env::set_var("DOCS_EXAMPLE_XDG_GREETING", "hello from a named app");
    }
    let loader = ConfigLoaderFactory::create_loader_xdg_with_substitution(
        "docs-example-app",
        Box::new(policy),
    )
    .expect("create_loader_xdg_with_substitution");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe {
        std::env::remove_var("SWE_EDGE_CONFIG_DIR");
        std::env::remove_var("DOCS_EXAMPLE_XDG_GREETING");
    }

    assert_eq!(cfg.greeting, "hello from a named app");
    println!(
        "create_loader_xdg_with_substitution: greeting={}",
        cfg.greeting
    );
}
