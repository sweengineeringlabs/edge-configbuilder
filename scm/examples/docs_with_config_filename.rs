//! `ConfigBuilder::with_config_filename(name)` — overrides the config
//! filename searched for in each configured directory. Defaults to
//! `application.toml` when never called.
//!
//! ## When to use this
//!
//! Every directory `configbuilder` searches — whether from
//! `with_config_dir`, XDG resolution, or the env var chain — is always
//! searched for a file with this filename. There's no per-directory
//! override; it's one filename for the whole builder. Use this when your
//! deployment convention names config files something other than
//! `application.toml` (e.g. `settings.toml`, or a filename matching an
//! existing ops convention) and you don't want to fork the crate to get it.
//!
//! ## What this example does
//!
//! 1. Writes *two* files into the same directory: a stray `application.toml`
//!    (the default filename, deliberately containing the wrong value) and a
//!    `settings.toml` (the one we'll actually configure).
//! 2. Chains `with_config_filename("settings.toml")` onto the builder.
//! 3. Loads the section and asserts the value came from `settings.toml` —
//!    proving the default `application.toml` was genuinely ignored, not
//!    just that `settings.toml` happened to also be read.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{BuilderFinalizer as _, ConfigBuilder as _, ConfigLoaderFactory, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    value: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    // A stray application.toml must be ignored once a custom filename is set.
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nvalue = \"wrong-file\"\n",
    )
    .expect("write application.toml");
    std::fs::write(
        dir.path().join("settings.toml"),
        "[app]\nvalue = \"right-file\"\n",
    )
    .expect("write settings.toml");

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .with_config_filename("settings.toml")
        .build_loader()
        .expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");

    assert_eq!(cfg.value, "right-file");
    println!(
        "with_config_filename: read settings.toml, value={}",
        cfg.value
    );
}
