//! Tests for `ConfigError` display formatting.

use swe_edge_configbuilder::ConfigError;

#[test]
fn test_config_error_display_parse() {
    let e = ConfigError::Parse("bad toml".into());
    assert!(e.to_string().contains("parse error"));
    assert!(e.to_string().contains("bad toml"));
}

#[test]
fn test_config_error_display_io() {
    let e = ConfigError::Io("permission denied".into());
    assert!(e.to_string().contains("io error"));
    assert!(e.to_string().contains("permission denied"));
}
