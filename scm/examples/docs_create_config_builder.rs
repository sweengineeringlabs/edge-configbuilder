//! `ConfigLoaderFactory::create_config_builder()` — the fluent builder-chain
//! entry point.
//!
//! ## When to use this
//!
//! Every `create_loader*` constructor is really a shortcut for a specific
//! builder configuration. Reach for the builder directly when you need to
//! combine multiple options that don't have a dedicated one-shot
//! constructor — e.g. an explicit directory *and* a custom filename *and* a
//! longer read timeout, all at once. `name`/`version` default to
//! `CARGO_PKG_NAME`/`CARGO_PKG_VERSION` until overridden — see the
//! `docs_with_name` and `docs_with_version` examples for that. See
//! `docs_with_config_dir`, `docs_with_config_filename`, and
//! `docs_with_read_timeout` for the other fluent setters this builder
//! exposes. For substitution support on the builder chain, use
//! `create_config_builder_with_substitution` instead (see
//! `docs_create_config_builder_with_substitution`).
//!
//! ## What this example does
//!
//! 1. Creates a temp directory and writes an `application.toml` into it.
//! 2. Chains `create_config_builder()` → `with_config_dir(dir)` →
//!    `build_loader()` — the minimal chain that produces a working loader.
//! 3. Loads the section and asserts the value round-trips correctly.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigLoaderFactory, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    name: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nname = \"create_config_builder-example\"\n",
    )
    .expect("write application.toml");

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .build_loader()
        .expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");

    assert_eq!(cfg.name, "create_config_builder-example");
    println!("create_config_builder: loaded name={}", cfg.name);
}
