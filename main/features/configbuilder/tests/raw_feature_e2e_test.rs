//! Tests for `RawFeature` — the type-erased feature load result.
#![allow(clippy::unwrap_used)]

use std::io::Write as _;
use swe_edge_configbuilder::{create_loader_for_dir, FeatureState};
use tempfile::TempDir;

fn dir_with(content: &str) -> TempDir {
    let dir = TempDir::new().unwrap();
    let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    dir
}

/// @covers: raw_feature::RawFeature
#[test]
fn test_raw_feature_present_section_results_in_enabled_state() {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct Feat {
        x: u32,
    }

    let dir = dir_with("[feat]\nx = 7");
    let state: FeatureState<Feat> = create_loader_for_dir(dir.path())
        .load_feature("feat")
        .unwrap()
        .state;
    assert!(state.is_enabled());
    assert_eq!(state.into_option().unwrap().x, 7);
}

/// @covers: raw_feature::RawFeature
#[test]
fn test_raw_feature_absent_section_results_in_disabled_state() {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct Feat {
        x: u32,
    }

    let dir = dir_with("[other]\nx = 1");
    let state: FeatureState<Feat> = create_loader_for_dir(dir.path())
        .load_feature("feat")
        .unwrap()
        .state;
    assert!(state.is_disabled());
}
