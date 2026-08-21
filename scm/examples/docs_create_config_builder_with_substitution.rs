//! `ConfigLoaderFactory::create_config_builder_with_substitution(policy)` —
//! the fluent builder-chain entry point, with `{{VAR_NAME}}` substitution
//! attached from construction.
//!
//! ## When to use this
//!
//! The substitution-enabled counterpart to `create_config_builder`
//! (`docs_create_config_builder`): use it when you need both the flexible
//! builder chain (custom directories, custom filename) *and* `{{VAR_NAME}}`
//! substitution, but don't need a custom `ValueResolver` — this returns a
//! `SubstitutionConfigBuilderImpl`, a distinct type from the plain
//! `ConfigBuilderImpl` `create_config_builder()` returns, and notably does
//! **not** support `with_read_timeout` (see `docs_with_read_timeout`) —
//! only the plain, non-substitution builder does.
//!
//! ## What this example does
//!
//! 1. Writes an `application.toml` with a
//!    `{{DOCS_EXAMPLE_BUILDER_GREETING}}` placeholder.
//! 2. Creates a `PrefixWhitelistPolicy` allowing only that name prefix.
//! 3. Sets the env var the placeholder resolves to.
//! 4. Chains `create_config_builder_with_substitution(policy)` →
//!    `with_config_dir(dir)` → `build_loader()`, loads the section, and
//!    asserts substitution happened.
#![allow(unsafe_code)]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigLoaderFactory, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    greeting: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\ngreeting = \"{{DOCS_EXAMPLE_BUILDER_GREETING}}\"\n",
    )
    .expect("write application.toml");

    let policy = ConfigLoaderFactory::create_prefix_whitelist_policy(vec![
        "DOCS_EXAMPLE_BUILDER_".to_string()
    ]);

    // SAFETY: single-threaded example process; no other thread touches this var.
    unsafe {
        std::env::set_var(
            "DOCS_EXAMPLE_BUILDER_GREETING",
            "hello from the builder chain",
        )
    };
    let loader = ConfigLoaderFactory::create_config_builder_with_substitution(Box::new(policy))
        .with_config_dir(dir.path())
        .build_loader()
        .expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");
    // SAFETY: cleanup — same invariant as above.
    unsafe { std::env::remove_var("DOCS_EXAMPLE_BUILDER_GREETING") };

    assert_eq!(cfg.greeting, "hello from the builder chain");
    println!(
        "create_config_builder_with_substitution: greeting={}",
        cfg.greeting
    );
}
