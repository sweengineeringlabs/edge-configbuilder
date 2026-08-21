//! `ConfigBuilder::with_config_dir(dir)` — appends an explicit config
//! directory to the builder chain.
//!
//! ## When to use this
//!
//! This is the builder-chain equivalent of `create_loader_for_dir`
//! (`docs_create_loader_for_dir`), but composable: call it multiple times to
//! build a *layered* directory chain (e.g. "defaults, then environment
//! overrides, then operator overrides") instead of a single fixed
//! directory. Setting even one explicit dir takes precedence over XDG/env
//! resolution entirely — see `DefaultConfigBuilder::build_loader_internal`'s
//! branch order in the crate source for the exact precedence rule.
//!
//! ## What this example does
//!
//! 1. Creates two temp directories, each with an `application.toml` setting
//!    the same key to a different value — `low` and `high`.
//! 2. Chains `with_config_dir(low)` then `with_config_dir(high)`.
//! 3. Loads the section and asserts the value from `high` won — later calls
//!    to `with_config_dir` take precedence on key conflicts, mirroring how
//!    later `$XDG_CONFIG_DIRS` entries win over earlier ones.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigLoaderFactory, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    value: String,
}

fn main() {
    let low = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        low.path().join("application.toml"),
        "[app]\nvalue = \"low-priority\"\n",
    )
    .expect("write low application.toml");

    let high = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        high.path().join("application.toml"),
        "[app]\nvalue = \"high-priority\"\n",
    )
    .expect("write high application.toml");

    // Later with_config_dir calls win on key conflicts.
    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(low.path())
        .with_config_dir(high.path())
        .build_loader()
        .expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");

    assert_eq!(cfg.value, "high-priority");
    println!("with_config_dir: later dir wins, value={}", cfg.value);
}
