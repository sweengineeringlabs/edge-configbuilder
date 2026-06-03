//! End-to-end tests for `SectionLoaderImpl`.
#![allow(clippy::unwrap_used)]
use swe_edge_configbuilder::ConfigLoaderFactory;

use std::io::Write as _;
use swe_edge_configbuilder::SectionLoaderImpl;
use tempfile::TempDir;

fn make_loader(content: &str) -> (TempDir, SectionLoaderImpl) {
    let dir = TempDir::new().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
}

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct Cfg {
    value: String,
}

/// @covers: section_loader_impl::SectionLoaderImpl::load_section
#[test]
fn test_section_loader_impl_load_section_returns_value_from_toml() {
    let (_dir, loader) = make_loader("[app]\nvalue = \"hello\"");
    let cfg: Cfg = loader.load_section("app").unwrap();
    assert_eq!(cfg.value, "hello");
}

/// @covers: section_loader_impl::SectionLoaderImpl::load_section
#[test]
fn test_section_loader_impl_load_section_absent_key_returns_default() {
    let (_dir, loader) = make_loader("[other]\nvalue = \"x\"");
    let cfg: Cfg = loader.load_section("app").unwrap();
    assert_eq!(cfg.value, "");
}

/// @covers: section_loader_impl::SectionLoaderImpl::validate
#[test]
fn test_section_loader_impl_validate_existing_dir_returns_ok() {
    let dir = TempDir::new().unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    assert!(loader.validate().is_ok());
}
