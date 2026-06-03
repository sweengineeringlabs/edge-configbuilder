//! Tests for FeatureState — enabled/disabled feature values.
use swe_edge_configbuilder::FeatureState;

// ── is_enabled / is_disabled ──────────────────────────────────────────────────

#[test]
fn test_feature_state_enum_is_enabled_returns_true_when_variant_is_enabled() {
    let state: FeatureState<u32> = FeatureState::Enabled(42);
    assert!(state.is_enabled());
}

#[test]
fn test_feature_state_enum_is_enabled_returns_false_when_variant_is_disabled() {
    let state: FeatureState<u32> = FeatureState::Disabled;
    assert!(!state.is_enabled());
}

#[test]
fn test_feature_state_enum_is_disabled_returns_true_when_variant_is_disabled() {
    let state: FeatureState<u32> = FeatureState::Disabled;
    assert!(state.is_disabled());
}

#[test]
fn test_feature_state_enum_is_disabled_returns_false_when_variant_is_enabled() {
    let state: FeatureState<u32> = FeatureState::Enabled(1);
    assert!(!state.is_disabled());
}

// ── into_option / as_option ──────────────────────────────────────────────────

#[test]
fn test_feature_state_enum_into_option_returns_some_when_enabled() {
    let state: FeatureState<i32> = FeatureState::Enabled(-7);
    assert_eq!(state.into_option(), Some(-7));
}

#[test]
fn test_feature_state_enum_into_option_returns_none_when_disabled() {
    let state: FeatureState<i32> = FeatureState::Disabled;
    assert_eq!(state.into_option(), None);
}

#[test]
fn test_feature_state_enum_as_option_returns_some_ref_when_enabled() {
    let state: FeatureState<String> = FeatureState::Enabled("hello".into());
    assert_eq!(state.as_option(), Some(&"hello".to_string()));
}

#[test]
fn test_feature_state_enum_as_option_returns_none_when_disabled() {
    let state: FeatureState<String> = FeatureState::Disabled;
    assert_eq!(state.as_option(), None);
}

// ── map ───────────────────────────────────────────────────────────────────────

#[test]
fn test_feature_state_enum_map_transforms_enabled_value() {
    let state: FeatureState<u32> = FeatureState::Enabled(3);
    let mapped = state.map(|n| n * 10);
    assert_eq!(mapped.into_option(), Some(30));
}

#[test]
fn test_feature_state_enum_map_propagates_disabled_unchanged() {
    let state: FeatureState<u32> = FeatureState::Disabled;
    let mapped = state.map(|n| n * 10);
    assert!(mapped.is_disabled());
}

// ── and_then ─────────────────────────────────────────────────────────────────

#[test]
fn test_feature_state_enum_and_then_chains_enabled_to_enabled() {
    let state: FeatureState<u32> = FeatureState::Enabled(5);
    let chained = state.and_then(|n| FeatureState::Enabled(n + 1));
    assert_eq!(chained.into_option(), Some(6));
}

#[test]
fn test_feature_state_enum_and_then_enabled_can_produce_disabled() {
    let state: FeatureState<u32> = FeatureState::Enabled(5);
    let chained = state.and_then(|_| FeatureState::<u32>::Disabled);
    assert!(chained.is_disabled());
}

#[test]
fn test_feature_state_enum_and_then_short_circuits_on_disabled() {
    let state: FeatureState<u32> = FeatureState::Disabled;
    let chained = state.and_then(|n| FeatureState::Enabled(n + 1));
    assert!(chained.is_disabled());
}

// ── unwrap_or / unwrap_or_else ────────────────────────────────────────────────

#[test]
fn test_feature_state_enum_unwrap_or_returns_inner_value_when_enabled() {
    let state: FeatureState<u32> = FeatureState::Enabled(99);
    assert_eq!(state.unwrap_or(0), 99);
}

#[test]
fn test_feature_state_enum_unwrap_or_returns_fallback_when_disabled() {
    let state: FeatureState<u32> = FeatureState::Disabled;
    assert_eq!(state.unwrap_or(42), 42);
}

#[test]
fn test_feature_state_enum_unwrap_or_else_returns_inner_value_when_enabled() {
    let state: FeatureState<u32> = FeatureState::Enabled(7);
    assert_eq!(state.unwrap_or_else(|| 0), 7);
}

#[test]
fn test_feature_state_enum_unwrap_or_else_calls_closure_when_disabled() {
    let state: FeatureState<u32> = FeatureState::Disabled;
    assert_eq!(state.unwrap_or_else(|| 100), 100);
}

// ── enabled_or_default ────────────────────────────────────────────────────────

#[test]
fn test_feature_state_enum_enabled_or_default_returns_value_when_enabled() {
    let state: FeatureState<String> = FeatureState::Enabled("cfg".into());
    assert_eq!(state.enabled_or_default(), "cfg");
}

#[test]
fn test_feature_state_enum_enabled_or_default_returns_default_when_disabled() {
    let state: FeatureState<u32> = FeatureState::Disabled;
    assert_eq!(state.enabled_or_default(), 0u32);
}

// ── From<Option<T>> / From<FeatureState<T>> for Option<T> ────────────────────

#[test]
fn test_feature_state_enum_from_some_creates_enabled() {
    let state: FeatureState<u32> = Some(5).into();
    assert_eq!(state.into_option(), Some(5));
}

#[test]
fn test_feature_state_enum_from_none_creates_disabled() {
    let state: FeatureState<u32> = None.into();
    assert!(state.is_disabled());
}

#[test]
fn test_feature_state_enum_option_from_enabled_state_is_some() {
    let state: FeatureState<u32> = FeatureState::Enabled(3);
    let opt: Option<u32> = state.into();
    assert_eq!(opt, Some(3));
}

#[test]
fn test_feature_state_enum_option_from_disabled_state_is_none() {
    let state: FeatureState<u32> = FeatureState::Disabled;
    let opt: Option<u32> = state.into();
    assert_eq!(opt, None);
}

// ── PartialEq ─────────────────────────────────────────────────────────────────

#[test]
fn test_feature_state_enum_eq_two_enabled_with_same_value_are_equal() {
    let a: FeatureState<u32> = FeatureState::Enabled(1);
    let b: FeatureState<u32> = FeatureState::Enabled(1);
    assert_eq!(a, b);
}

#[test]
fn test_feature_state_enum_eq_two_disabled_are_equal() {
    let a: FeatureState<u32> = FeatureState::Disabled;
    let b: FeatureState<u32> = FeatureState::Disabled;
    assert_eq!(a, b);
}

#[test]
fn test_feature_state_enum_eq_enabled_and_disabled_are_not_equal() {
    let a: FeatureState<u32> = FeatureState::Enabled(1);
    let b: FeatureState<u32> = FeatureState::Disabled;
    assert_ne!(a, b);
}
