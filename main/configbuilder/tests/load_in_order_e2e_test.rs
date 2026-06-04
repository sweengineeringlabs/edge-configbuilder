//! End-to-end tests for the `load_in_order!` macro.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use serde::Deserialize;
use swe_edge_configbuilder::{
    load_in_order, ConfigError, ConfigLoaderFactory, FeatureRegistry, OptionalSection,
};
use tempfile::TempDir;

fn write_toml(dir: &std::path::Path, content: &str) {
    std::fs::write(dir.join("application.toml"), content).unwrap();
}

// ── test section types ────────────────────────────────────────────────────────

#[derive(Deserialize, Default)]
struct AlphaSection {}

impl OptionalSection for AlphaSection {
    fn section_name() -> &'static str {
        "alpha"
    }
}

#[derive(Deserialize, Default)]
struct BetaSection {}

impl OptionalSection for BetaSection {
    fn section_name() -> &'static str {
        "beta"
    }
    fn requires() -> &'static [&'static str] {
        &["alpha"]
    }
}

#[derive(Deserialize, Default)]
struct GammaSection {}

impl OptionalSection for GammaSection {
    fn section_name() -> &'static str {
        "gamma"
    }
    fn requires() -> &'static [&'static str] {
        &["beta"]
    }
}

// Cycle pair: each requires the other
#[derive(Deserialize, Default)]
struct CycleX {}
impl OptionalSection for CycleX {
    fn section_name() -> &'static str {
        "cycle_x"
    }
    fn requires() -> &'static [&'static str] {
        &["cycle_y"]
    }
}

#[derive(Deserialize, Default)]
struct CycleY {}
impl OptionalSection for CycleY {
    fn section_name() -> &'static str {
        "cycle_y"
    }
    fn requires() -> &'static [&'static str] {
        &["cycle_x"]
    }
}

// ── basic happy path ──────────────────────────────────────────────────────────

#[test]
fn test_load_in_order_single_section_no_deps_succeeds() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let result = load_in_order!(&mut registry, &loader, AlphaSection);
    assert!(result.is_ok());
    assert_eq!(registry.records().len(), 1);
    assert_eq!(registry.records()[0].section_name, "alpha");
}

#[test]
fn test_load_in_order_multiple_independent_sections_all_recorded() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let result = load_in_order!(
        &mut registry,
        &loader,
        AlphaSection,
        GammaSection,
        BetaSection
    );
    assert!(result.is_ok(), "load failed: {result:?}");
    assert_eq!(registry.records().len(), 3);
}

// ── topological ordering ──────────────────────────────────────────────────────

#[test]
fn test_load_in_order_dependency_loads_before_dependent_regardless_of_macro_order() {
    // Macro lists BetaSection first, but BetaSection requires AlphaSection.
    // The toposort must put AlphaSection first in the registry records.
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    load_in_order!(&mut registry, &loader, BetaSection, AlphaSection).unwrap();

    let records = registry.records();
    let pos_alpha = records
        .iter()
        .position(|r| r.section_name == "alpha")
        .unwrap();
    let pos_beta = records
        .iter()
        .position(|r| r.section_name == "beta")
        .unwrap();
    assert!(
        pos_alpha < pos_beta,
        "alpha must be loaded before beta (beta depends on alpha)"
    );
}

#[test]
fn test_load_in_order_chain_loads_in_dependency_order() {
    // gamma → beta → alpha: all three listed in reverse dependency order
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    load_in_order!(
        &mut registry,
        &loader,
        GammaSection,
        BetaSection,
        AlphaSection
    )
    .unwrap();

    let records = registry.records();
    let pos = |name: &str| records.iter().position(|r| r.section_name == name).unwrap();
    assert!(pos("alpha") < pos("beta"), "alpha before beta");
    assert!(pos("beta") < pos("gamma"), "beta before gamma");
}

// ── cycle detection ───────────────────────────────────────────────────────────

#[test]
fn test_load_in_order_cycle_returns_validation_error() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let result = load_in_order!(&mut registry, &loader, CycleX, CycleY);
    assert!(result.is_err(), "cycle must produce an error");
    assert!(
        matches!(result, Err(ConfigError::Validation { .. })),
        "error must be ConfigError::Validation, got {result:?}"
    );
}

#[test]
fn test_load_in_order_cycle_error_section_is_load_in_order() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let err = load_in_order!(&mut registry, &loader, CycleX, CycleY).unwrap_err();
    match err {
        ConfigError::Validation { section, reason } => {
            assert_eq!(section, "load_in_order");
            assert!(
                reason.contains("cycle"),
                "reason must mention cycle: {reason}"
            );
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn test_load_in_order_cycle_leaves_registry_empty() {
    let dir = TempDir::new().unwrap();
    write_toml(dir.path(), "");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());

    let mut registry = FeatureRegistry::new();
    let _ = load_in_order!(&mut registry, &loader, CycleX, CycleY);
    assert_eq!(
        registry.records().len(),
        0,
        "no records must be stored when cycle aborts loading"
    );
}

// ── result propagation ────────────────────────────────────────────────────────

#[test]
fn test_load_in_order_result_is_compatible_with_question_mark() {
    fn try_load() -> Result<(), ConfigError> {
        let dir = TempDir::new().unwrap();
        write_toml(dir.path(), "");
        let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
        let mut registry = FeatureRegistry::new();
        load_in_order!(&mut registry, &loader, AlphaSection)?;
        Ok(())
    }
    assert!(try_load().is_ok());
}
