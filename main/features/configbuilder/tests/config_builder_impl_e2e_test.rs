//! End-to-end tests for `ConfigBuilderImpl`.
#![allow(clippy::unwrap_used)]

use std::io::Write as _;
use swe_edge_configbuilder::ConfigLoaderFactory;
/// @covers: config_builder_impl::ConfigBuilderImpl::with_name
#[test]
fn test_config_builder_impl_with_name_is_reflected() {
    let b = ConfigLoaderFactory::ConfigLoaderFactory::create_config_builder().with_name("my-app");
    assert_eq!(b.name(), "my-app");
}

/// @covers: config_builder_impl::ConfigBuilderImpl::with_version
#[test]
fn test_config_builder_impl_with_version_is_reflected() {
    let b = ConfigLoaderFactory::ConfigLoaderFactory::create_config_builder().with_version("2.0.0");
    assert_eq!(b.version(), "2.0.0");
}

/// @covers: config_builder_impl::ConfigBuilderImpl::build_loader
#[test]
fn test_config_builder_impl_build_loader_reads_explicit_dir() {
    let dir = tempfile::tempdir().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    writeln!(f, "[svc]\nkey = \"val\"").unwrap();

    #[derive(serde::Deserialize, Default, PartialEq, Debug)]
    #[serde(default)]
    struct Svc {
        key: String,
    }

    let loader = ConfigLoaderFactory::ConfigLoaderFactory::create_config_builder()
        .with_config_dir(dir.path())
        .build_loader()
        .unwrap();
    let cfg: Svc = loader.load_section("svc").unwrap();
    assert_eq!(cfg.key, "val");
}
