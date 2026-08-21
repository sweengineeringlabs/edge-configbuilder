//! `ConfigLoaderFactory::create_loader_xdg_with_substitution(app_name, policy)`
//! — `create_loader_xdg`'s app-namespaced XDG resolution, plus `{{VAR_NAME}}`
//! env var substitution in loaded TOML values.
//!
//! ## When to use this
//!
//! The substitution-enabled counterpart to `create_loader_xdg`
//! (`docs_create_loader_xdg`): the constructor most production services
//! should reach for when they both (a) have a stable app name for XDG
//! namespacing and (b) need to inject deploy-time values into config via
//! `{{VAR_NAME}}` placeholders, e.g. a database host that differs between
//! staging and production without maintaining separate TOML files.
//!
//! ## What this example does
//!
//! 1. Writes an `application.toml` with a `{{DOCS_EXAMPLE_XDG_GREETING}}`
//!    placeholder.
//! 2. Creates a `PrefixWhitelistPolicy` allowing only that name prefix.
//! 3. Points `$CONFIGBUILDER_CONFIG_DIR` at the temp dir (for determinism —
//!    see `docs_create_loader_xdg` for why this doesn't get joined with the
//!    app name) and sets the placeholder's actual value.
//! 4. Loads the section via the named app `"docs-example-app"` and asserts
//!    substitution happened correctly.
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
        std::env::set_var("CONFIGBUILDER_CONFIG_DIR", dir.path());
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
        std::env::remove_var("CONFIGBUILDER_CONFIG_DIR");
        std::env::remove_var("DOCS_EXAMPLE_XDG_GREETING");
    }

    assert_eq!(cfg.greeting, "hello from a named app");
    println!(
        "create_loader_xdg_with_substitution: greeting={}",
        cfg.greeting
    );
}
