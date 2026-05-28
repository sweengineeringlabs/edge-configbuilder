//! Contract tests for the section loader boundary constants.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_configbuilder::{create_loader_for_dir, Loader as _};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
#[serde(default)]
struct Sec {
    value: String,
}

/// @covers: api/default_section_loader::MAX_CONFIG_FILE_BYTES
#[test]
fn test_load_section_from_rejects_file_at_one_mib_plus_one_byte() {
    let dir = tempfile::tempdir().unwrap();
    let oversized = vec![b'#'; 1_048_577]; // 1 MiB + 1
    std::fs::write(dir.path().join("application.toml"), &oversized).unwrap();
    let err = create_loader_for_dir(dir.path())
        .load_section::<Sec>("s")
        .unwrap_err();
    assert!(
        err.to_string().contains("1 MiB"),
        "error must mention the 1 MiB limit: {err}"
    );
}

/// @covers: api/default_section_loader::MAX_CONFIG_FILE_BYTES
#[test]
fn test_load_section_from_accepts_file_at_exactly_one_mib() {
    let dir = tempfile::tempdir().unwrap();
    // '#' is a TOML comment — exactly 1 MiB of comments is valid TOML, no sections.
    let at_limit = vec![b'#'; 1_048_576];
    std::fs::write(dir.path().join("application.toml"), &at_limit).unwrap();
    let result: Result<Sec, _> = create_loader_for_dir(dir.path()).load_section("s");
    assert!(
        result.is_ok(),
        "file at the 1 MiB limit must not be rejected: {result:?}"
    );
    assert_eq!(result.unwrap(), Sec::default());
}

/// @covers: api/default_section_loader::FALLBACK_CONFIG_DIR
#[test]
fn test_load_section_without_env_var_returns_not_found_for_absent_section() {
    // Point SWE_EDGE_CONFIG_DIR to an empty temp dir so there is no
    // application.toml — the loader must return NotFound, not Ok(Default).
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("SWE_EDGE_CONFIG_DIR", dir.path().to_str().unwrap());
    let result: Result<Sec, _> = swe_edge_configbuilder::create_loader()
        .unwrap()
        .load_section("nonexistent_xyz");
    std::env::remove_var("SWE_EDGE_CONFIG_DIR");
    assert!(
        matches!(
            result,
            Err(swe_edge_configbuilder::ConfigError::NotFound(_))
        ),
        "config dir with no application.toml must return NotFound: {result:?}"
    );
}
