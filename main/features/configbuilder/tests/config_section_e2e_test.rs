// @covers: api/traits/config/section.rs — ConfigSection trait behaviour
use swe_edge_configbuilder::{create_loader_for_dir, ConfigSection};

use std::io::Write as _;
use tempfile::TempDir;

fn write_toml(dir: &std::path::Path, content: &str) {
    let mut f = std::fs::File::create(dir.join("application.toml")).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

#[derive(Debug, serde::Deserialize, Default, PartialEq)]
#[serde(default)]
struct AppSection {
    host: String,
    port: u16,
}

impl ConfigSection for AppSection {
    fn section_name() -> &'static str {
        "app"
    }
}

#[test]
fn test_config_section_load_absent_section_returns_default() {
    // ConfigSection::load must return Default when the key is absent from an
    // existing file — it must NOT return an error or expose internal paths.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[other]\nhost = \"unrelated\"");
    let loader = create_loader_for_dir(dir.path());
    let result = AppSection::load(&loader);
    assert!(
        result.is_ok(),
        "absent section must not error; got {result:?}"
    );
    assert_eq!(
        result.unwrap(),
        AppSection::default(),
        "absent section must equal Default"
    );
}

#[test]
fn test_config_section_load_present_section_returns_values() {
    // ConfigSection::load must deserialise the section when the key is present.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[app]\nhost = \"localhost\"\nport = 8080");
    let loader = create_loader_for_dir(dir.path());
    let result = AppSection::load(&loader);
    assert!(
        result.is_ok(),
        "present section must not error; got {result:?}"
    );
    let section = result.unwrap();
    assert_eq!(section.host, "localhost");
    assert_eq!(section.port, 8080);
}

#[test]
fn test_config_section_section_name_returns_correct_key() {
    // section_name() is the static contract between the struct and TOML.
    // Changing it would silently break all deployments that set [app] in TOML.
    assert_eq!(AppSection::section_name(), "app");
}

#[test]
fn test_config_section_load_parse_error_propagates() {
    // ConfigSection::load must propagate a parse error when the section exists
    // but the TOML is malformed — never silently swallow it.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "[app]\nport = \"not_a_number\"");
    let loader = create_loader_for_dir(dir.path());
    let result = AppSection::load(&loader);
    assert!(
        result.is_err(),
        "malformed field type must produce a parse error"
    );
}
