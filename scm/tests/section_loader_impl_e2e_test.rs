//! End-to-end tests for `SectionLoaderImpl`.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use swe_edge_configbuilder::ConfigLoaderFactory;

use std::io::Write as _;
use swe_edge_configbuilder::{Loader, SectionLoaderImpl};
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
    let cfg: Cfg = Loader::load_section(&loader, "app").unwrap();
    assert_eq!(cfg.value, "hello");
}

/// @covers: section_loader_impl::SectionLoaderImpl::load_section
#[test]
fn test_section_loader_impl_load_section_absent_key_returns_default() {
    let (_dir, loader) = make_loader("[other]\nvalue = \"x\"");
    let cfg: Cfg = Loader::load_section(&loader, "app").unwrap();
    assert_eq!(cfg.value, "");
}

/// @covers: section_loader_impl::SectionLoaderImpl::validate
#[test]
fn test_section_loader_impl_validate_existing_dir_returns_ok() {
    let dir = TempDir::new().unwrap();
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    assert!(Loader::validate(&loader).is_ok());
}

/// @covers: DefaultSectionLoader::read_with_timeout
///
/// Verifies that a stalled filesystem read returns `ConfigError::Io` with a
/// "timed out" message within the configured deadline.
///
/// Uses a named pipe (FIFO) as the config file: opening it for read blocks
/// until a writer appears, reliably simulating a hung NFS/FUSE mount.
/// This test is Unix-only because Windows named-pipe setup requires WinAPI.
#[cfg(unix)]
#[test]
fn test_load_section_times_out_on_stalled_read() {
    use std::time::Duration;
    use swe_edge_configbuilder::ConfigError;

    let dir = TempDir::new().unwrap();
    let toml_path = dir.path().join("application.toml");

    // Create a named pipe — reading it blocks until a writer opens the other end.
    let status = std::process::Command::new("mkfifo")
        .arg(&toml_path)
        .status()
        .expect("mkfifo must be available on this Unix system");
    assert!(status.success(), "mkfifo failed to create named pipe");

    let loader = swe_edge_configbuilder::ConfigBuilderImpl::new()
        .with_config_dir(dir.path())
        .with_read_timeout(Duration::from_millis(100))
        .build_loader()
        .expect("loader creation must succeed");

    let err = Loader::load_section::<Cfg>(&loader, "app").unwrap_err();
    assert!(
        matches!(err, ConfigError::Io(_)),
        "expected Io error on stalled read, got {err:?}"
    );
    assert!(
        err.to_string().contains("timed out"),
        "error must mention timeout: {err}"
    );
}
