//! End-to-end tests for `SubstitutionConfigBuilderImpl`.
#![allow(clippy::unwrap_used)]
#![allow(unsafe_code)]

use std::io::Write as _;
use swe_edge_configbuilder::{AllowAllPolicy, ConfigLoaderFactory};

/// @covers: substitution_config_builder_impl::SubstitutionConfigBuilderImpl::build_loader
#[test]
fn test_substitution_config_builder_impl_build_loader_applies_policy() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    f.write_all(b"[app]\nurl = \"http://{{HOST}}\"\n").unwrap();

    #[derive(serde::Deserialize, Default, PartialEq, Debug)]
    #[serde(default)]
    struct App {
        url: String,
    }

    // SAFETY: single-threaded test binary; no concurrent thread reads this env var
    unsafe { std::env::set_var("HOST", "localhost") };
    let loader =
        ConfigLoaderFactory::create_config_builder_with_substitution(Box::new(AllowAllPolicy))
            .with_config_dir(dir.path())
            .build_loader()
            .unwrap();
    let cfg: App = loader.load_section("app").unwrap();
    // SAFETY: cleanup — same invariant as above
    unsafe { std::env::remove_var("HOST") };

    assert_eq!(cfg.url, "http://localhost");
}
