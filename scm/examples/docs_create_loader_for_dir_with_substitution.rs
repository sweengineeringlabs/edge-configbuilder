//! `ConfigLoaderFactory::create_loader_for_dir_with_substitution(dir, policy)`
//! — `create_loader_for_dir`'s no-resolution simplicity, plus `{{VAR_NAME}}`
//! env var substitution in loaded TOML values.
//!
//! ## When to use this
//!
//! The substitution-enabled counterpart to `create_loader_for_dir`
//! (`docs_create_loader_for_dir`): use it when the config directory is
//! already known (no XDG/env resolution needed) but you still want to
//! inject deploy-time values via `{{VAR_NAME}}` placeholders. It's the only
//! one of the three `_with_substitution` constructors that never touches env
//! vars for *directory* resolution — only for the substituted values
//! themselves — which makes it the most deterministic of the three to test
//! and demonstrate.
//!
//! ## What this example does
//!
//! 1. Writes an `application.toml` with a `{{DOCS_EXAMPLE_DIR_GREETING}}`
//!    placeholder.
//! 2. Creates a `PrefixWhitelistPolicy` allowing only that name prefix.
//! 3. Sets the actual env var the placeholder resolves to (no
//!    `$CONFIGBUILDER_CONFIG_DIR` needed here — the directory is passed
//!    directly).
//! 4. Loads the section and asserts substitution happened correctly.
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
        "[app]\ngreeting = \"{{DOCS_EXAMPLE_DIR_GREETING}}\"\n",
    )
    .expect("write application.toml");

    let policy =
        ConfigLoaderFactory::create_prefix_whitelist_policy(vec!["DOCS_EXAMPLE_DIR_".to_string()]);

    // SAFETY: single-threaded example process; no other thread touches this var.
    unsafe { std::env::set_var("DOCS_EXAMPLE_DIR_GREETING", "hello from a fixed dir") };
    let loader =
        ConfigLoaderFactory::create_loader_for_dir_with_substitution(dir.path(), Box::new(policy));
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("DOCS_EXAMPLE_DIR_GREETING") };

    assert_eq!(cfg.greeting, "hello from a fixed dir");
    println!(
        "create_loader_for_dir_with_substitution: greeting={}",
        cfg.greeting
    );
}
