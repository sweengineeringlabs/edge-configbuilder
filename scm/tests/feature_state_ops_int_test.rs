//! Integration tests for `FeatureStateOps` trait methods.
#![allow(missing_docs)]
use swe_edge_configbuilder::{FeatureState, FeatureStateOps as _};

// ── is_enabled ────────────────────────────────────────────────────────────────

#[test]
fn test_is_enabled_returns_true_for_enabled_state_happy() {
    let s: FeatureState<u32> = FeatureState::Enabled(1);
    assert!(s.is_enabled());
}

#[test]
fn test_is_enabled_returns_false_for_disabled_state_error() {
    let s: FeatureState<u32> = FeatureState::Disabled;
    assert!(!s.is_enabled());
}

#[test]
fn test_is_enabled_zero_value_still_counts_as_enabled_edge() {
    let s: FeatureState<u32> = FeatureState::Enabled(0);
    assert!(s.is_enabled());
}

// ── is_disabled ───────────────────────────────────────────────────────────────

#[test]
fn test_is_disabled_returns_true_for_disabled_state_happy() {
    let s: FeatureState<u32> = FeatureState::Disabled;
    assert!(s.is_disabled());
}

#[test]
fn test_is_disabled_returns_false_for_enabled_state_error() {
    let s: FeatureState<u32> = FeatureState::Enabled(99);
    assert!(!s.is_disabled());
}

#[test]
fn test_is_disabled_and_is_enabled_are_mutually_exclusive_edge() {
    let e: FeatureState<u32> = FeatureState::Enabled(1);
    let d: FeatureState<u32> = FeatureState::Disabled;
    assert_ne!(e.is_enabled(), e.is_disabled());
    assert_ne!(d.is_enabled(), d.is_disabled());
}

// ── into_option ───────────────────────────────────────────────────────────────

#[test]
fn test_into_option_returns_some_when_enabled_happy() {
    let s: FeatureState<i32> = FeatureState::Enabled(42);
    assert_eq!(s.into_option(), Some(42));
}

#[test]
fn test_into_option_returns_none_when_disabled_error() {
    let s: FeatureState<i32> = FeatureState::Disabled;
    assert_eq!(s.into_option(), None);
}

#[test]
fn test_into_option_consumes_the_state_edge() {
    let s: FeatureState<String> = FeatureState::Enabled("hello".into());
    let opt = s.into_option();
    assert_eq!(opt.as_deref(), Some("hello"));
}

// ── as_option ─────────────────────────────────────────────────────────────────

#[test]
fn test_as_option_returns_some_ref_when_enabled_happy() {
    let s: FeatureState<u32> = FeatureState::Enabled(7);
    assert_eq!(s.as_option(), Some(&7));
}

#[test]
fn test_as_option_returns_none_when_disabled_error() {
    let s: FeatureState<u32> = FeatureState::Disabled;
    assert!(s.as_option().is_none());
}

#[test]
fn test_as_option_borrows_without_moving_edge() {
    let s: FeatureState<u32> = FeatureState::Enabled(3);
    let _ = s.as_option();
    // s is still accessible
    assert!(s.is_enabled());
}

// ── map ───────────────────────────────────────────────────────────────────────

#[test]
fn test_map_transforms_inner_value_when_enabled_happy() {
    let s: FeatureState<u32> = FeatureState::Enabled(5);
    assert_eq!(s.map(|v| v * 2).into_option(), Some(10));
}

#[test]
fn test_map_propagates_disabled_without_calling_closure_error() {
    let s: FeatureState<u32> = FeatureState::Disabled;
    let called = std::cell::Cell::new(false);
    let result = s.map(|v| { called.set(true); v });
    assert!(!called.get());
    assert!(result.is_disabled());
}

#[test]
fn test_map_can_change_inner_type_edge() {
    let s: FeatureState<u32> = FeatureState::Enabled(3);
    let s2: FeatureState<String> = s.map(|v| v.to_string());
    assert_eq!(s2.into_option(), Some("3".to_string()));
}

// ── and_then ─────────────────────────────────────────────────────────────────

#[test]
fn test_and_then_chains_to_enabled_when_both_enabled_happy() {
    let s: FeatureState<u32> = FeatureState::Enabled(3);
    let r = s.and_then(|v| FeatureState::Enabled(v + 10));
    assert_eq!(r.into_option(), Some(13));
}

#[test]
fn test_and_then_short_circuits_on_disabled_input_error() {
    let s: FeatureState<u32> = FeatureState::Disabled;
    let r = s.and_then(|v| FeatureState::Enabled(v + 10));
    assert!(r.is_disabled());
}

#[test]
fn test_and_then_enabled_input_can_produce_disabled_edge() {
    let s: FeatureState<u32> = FeatureState::Enabled(0);
    let r = s.and_then(|_| FeatureState::<u32>::Disabled);
    assert!(r.is_disabled());
}

// ── unwrap_or ─────────────────────────────────────────────────────────────────

#[test]
fn test_unwrap_or_returns_inner_value_when_enabled_happy() {
    let s: FeatureState<u32> = FeatureState::Enabled(42);
    assert_eq!(s.unwrap_or(0), 42);
}

#[test]
fn test_unwrap_or_returns_fallback_when_disabled_error() {
    let s: FeatureState<u32> = FeatureState::Disabled;
    assert_eq!(s.unwrap_or(99), 99);
}

#[test]
fn test_unwrap_or_zero_fallback_when_disabled_edge() {
    let s: FeatureState<u32> = FeatureState::Disabled;
    assert_eq!(s.unwrap_or(0), 0);
}

// ── unwrap_or_else ────────────────────────────────────────────────────────────

#[test]
fn test_unwrap_or_else_returns_value_when_enabled_happy() {
    let s: FeatureState<u32> = FeatureState::Enabled(5);
    assert_eq!(s.unwrap_or_else(|| 100), 5);
}

#[test]
fn test_unwrap_or_else_calls_closure_when_disabled_error() {
    let s: FeatureState<u32> = FeatureState::Disabled;
    assert_eq!(s.unwrap_or_else(|| 77), 77);
}

#[test]
fn test_unwrap_or_else_closure_not_called_when_enabled_edge() {
    let s: FeatureState<u32> = FeatureState::Enabled(1);
    let called = std::cell::Cell::new(false);
    s.unwrap_or_else(|| { called.set(true); 0 });
    assert!(!called.get(), "closure must not be invoked when state is Enabled");
}

// ── enabled_or_default ────────────────────────────────────────────────────────

#[test]
fn test_enabled_or_default_returns_value_when_enabled_happy() {
    let s: FeatureState<u32> = FeatureState::Enabled(7);
    assert_eq!(s.enabled_or_default(), 7);
}

#[test]
fn test_enabled_or_default_returns_type_default_when_disabled_error() {
    let s: FeatureState<u32> = FeatureState::Disabled;
    assert_eq!(s.enabled_or_default(), 0u32);
}

#[test]
fn test_enabled_or_default_string_default_is_empty_when_disabled_edge() {
    let s: FeatureState<String> = FeatureState::Disabled;
    assert_eq!(s.enabled_or_default(), String::new());
}
