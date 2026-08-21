//! `ConfigLoaderFactory::create_loader_with_substitution(policy)` —
//! `create_loader()`'s XDG resolution, plus `{{VAR_NAME}}` env var
//! substitution in loaded TOML values.
//!
//! ## When to use this
//!
//! Use it when config values need to be injected at deploy time (hostnames,
//! ports, feature flags) rather than hardcoded in the TOML file, but you
//! don't want secrets management beyond `std::env::var`. The `policy`
//! argument is mandatory and controls *which* variable names are allowed to
//! be substituted — substitution is opt-in per name, not a blanket "replace
//! anything in braces". This example uses `PrefixWhitelistPolicy`, the
//! crate's recommended production default; `AllowAllPolicy` exists too but
//! is gated behind the `test-utils` feature and must never reach production.
//!
//! For the same substitution behavior with an explicit directory instead of
//! XDG resolution, see `docs_create_loader_for_dir_with_substitution`.
//!
//! ## What this example does
//!
//! 1. Writes an `application.toml` whose value is a `{{DOCS_EXAMPLE_GREETING}}`
//!    placeholder rather than a literal string.
//! 2. Creates a policy that only allows names starting with `DOCS_EXAMPLE_`.
//! 3. Points `$CONFIGBUILDER_CONFIG_DIR` at the temp dir (for determinism,
//!    same as `docs_create_loader`) and sets the actual env var the
//!    placeholder resolves to.
//! 4. Loads the section and asserts the placeholder was replaced with the
//!    env var's value, proving the whole substitution pipeline worked.
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

    // Only names with this prefix may be substituted; everything else is rejected.
    let policy =
        ConfigLoaderFactory::create_prefix_whitelist_policy(vec!["DOCS_EXAMPLE_".to_string()]);

    // SAFETY: single-threaded example process; no other thread touches these vars.
    unsafe {
        std::env::set_var("CONFIGBUILDER_CONFIG_DIR", dir.path());
        std::env::set_var("DOCS_EXAMPLE_GREETING", "hello from substitution");
    }
    let loader = ConfigLoaderFactory::create_loader_with_substitution(Box::new(policy))
        .expect("create_loader_with_substitution");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe {
        std::env::remove_var("CONFIGBUILDER_CONFIG_DIR");
        std::env::remove_var("DOCS_EXAMPLE_GREETING");
    }

    assert_eq!(cfg.greeting, "hello from substitution");
    println!("create_loader_with_substitution: greeting={}", cfg.greeting);
}
