//! Integration tests for [`ConfigLoaderFactory`].

use swe_edge_configbuilder::ConfigLoaderFactory;

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_returns_builder_that_can_build_loader() {
    let _loader = ConfigLoaderFactory::create_config_builder().build_loader();
}

/// @covers: create_loader_for_dir
#[test]
fn test_create_loader_for_dir_accepts_temp_dir() {
    let dir = std::env::temp_dir();
    let _loader = ConfigLoaderFactory::create_loader_for_dir(dir);
}

/// @covers: create_validator
#[test]
fn test_create_validator_returns_path_validator() {
    let _v = ConfigLoaderFactory::create_validator();
}

/// @covers: load_section_xdg
///
/// Tests both present-section (returns value) and absent-section (returns Default)
/// in a single test to avoid env-var races when tests run in parallel.
#[test]
fn test_load_section_xdg_reads_section_and_returns_default_when_absent(
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write as _;

    #[derive(serde::Deserialize, Default, Debug, PartialEq)]
    struct GoalSection {
        target: String,
    }
    #[derive(serde::Deserialize, Default, PartialEq, Debug)]
    struct OtherSection {
        value: u32,
    }

    let dir = tempfile::tempdir()?;
    std::env::set_var("SWE_EDGE_CONFIG_DIR", dir.path());

    let mut f = std::fs::File::create(dir.path().join("application.toml"))?;
    f.write_all(b"[goal]\ntarget = \"prod\"\n")?;

    // Present section → reads value.
    let goal = ConfigLoaderFactory::load_section_xdg::<GoalSection>("dummy", "goal")?;
    assert_eq!(goal.target, "prod");

    // Absent section → file exists but key missing → Default.
    let absent = ConfigLoaderFactory::load_section_xdg::<OtherSection>("dummy", "missing")?;
    assert_eq!(
        absent,
        OtherSection { value: 0 },
        "absent section must return default"
    );

    std::env::remove_var("SWE_EDGE_CONFIG_DIR");
    Ok(())
}
