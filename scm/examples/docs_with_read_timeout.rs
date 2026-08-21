//! `ConfigBuilderInit::with_read_timeout(duration)` — overrides the
//! 30-second default wall-clock deadline for each config file read.
//!
//! ## When to use this
//!
//! Guards against a single file read hanging startup forever — e.g. a
//! stalled NFS/FUSE mount, or a network filesystem under load. 30 seconds
//! (`DEFAULT_READ_TIMEOUT` in the crate source) is generous enough for a
//! slow spinning disk while still bounding the worst case. Lower it if you
//! want startup to fail fast on a misbehaving mount; raise it if your
//! storage is known to be reliably slow rather than actually stalled.
//!
//! **Availability note:** this setter is only on `ConfigBuilderImpl` (via
//! the `ConfigBuilderInit` trait) — the plain builder from
//! `create_config_builder()`. `SubstitutionConfigBuilderImpl` (from
//! `create_config_builder_with_substitution`, see
//! `docs_create_config_builder_with_substitution`) does **not** expose it.
//!
//! ## What this example does
//!
//! A generous timeout has no observable effect on a normal, fast local
//! read — there's no practical way to demonstrate the *failure* path (an
//! actually-stalled read) in a portable example. So this example proves the
//! non-obvious-but-important thing instead: setting a real, finite timeout
//! doesn't break the happy path. It sets a 5-second deadline, reads a
//! normal file, and asserts the read still succeeds.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use configbuilder::{
    BuilderFinalizer as _, ConfigBuilder as _, ConfigBuilderInit as _, ConfigLoaderFactory,
    Loader as _,
};
use std::time::Duration;

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct AppConfig {
    value: String,
}

fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        dir.path().join("application.toml"),
        "[app]\nvalue = \"with_read_timeout-example\"\n",
    )
    .expect("write application.toml");

    let loader = ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .with_read_timeout(Duration::from_secs(5))
        .build_loader()
        .expect("build_loader");
    let cfg: AppConfig = loader.load_section("app").expect("load app section");

    assert_eq!(cfg.value, "with_read_timeout-example");
    println!(
        "with_read_timeout: 5s deadline, normal read still succeeds, value={}",
        cfg.value
    );
}
