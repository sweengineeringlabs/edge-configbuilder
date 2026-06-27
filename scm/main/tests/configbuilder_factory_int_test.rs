use swe_edge_configbuilder::{ConfigLoaderFactory, Validator as _, PreflightReportOps as _, FeatureRegistryOps as _, Loader as _, TopologyOps as _};

#[test]
fn test_create_loader_happy() {
    let loader_result = ConfigLoaderFactory::create_loader();
    let _ = loader_result.expect("create_loader");
}

#[test]
fn test_create_validator_happy() {
    let validator = ConfigLoaderFactory::create_validator();
    let path = std::path::Path::new(".");
    let result = validator.validate_path(path);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_create_preflight_report_happy() {
    let report = ConfigLoaderFactory::create_preflight_report();
    assert_eq!(report.issue_count(), 0);
    assert!(report.is_ok());
}

#[test]
fn test_create_feature_registry_happy() {
    let registry = ConfigLoaderFactory::create_feature_registry();
    assert_eq!(registry.records().len(), 0);
    let summary = registry.summary();
    assert_eq!(summary.total_count(), 0);
}

#[test]
fn test_create_prefix_whitelist_policy_happy() {
    let prefixes = vec!["APP_".to_string(), "TEST_".to_string()];
    let policy = ConfigLoaderFactory::create_prefix_whitelist_policy(prefixes);
    assert_eq!(policy.prefixes.len(), 2);
    assert_eq!(policy.prefixes[0], "APP_");
}

#[test]
fn test_create_pattern_whitelist_policy_happy() {
    let pattern = r"^APP_[A-Z_]+$".to_string();
    let policy = ConfigLoaderFactory::create_pattern_whitelist_policy(pattern)
        .expect("pattern");
    assert!(!policy.pattern_str.is_empty());
}

#[test]
fn test_create_pattern_whitelist_policy_error() {
    let invalid_pattern = "[invalid".to_string();
    let result = ConfigLoaderFactory::create_pattern_whitelist_policy(invalid_pattern);
    assert!(result.is_err());
}

#[test]
fn test_create_composite_policy_happy() {
    let policies: Vec<Box<dyn swe_edge_configbuilder::SubstitutionPolicy>> = vec![];
    let policy = ConfigLoaderFactory::create_composite_policy(policies);
    assert_eq!(policy.policies.len(), 0);
}

#[test]
fn test_topology_sort_happy() {
    let names = vec!["a", "b", "c"];
    let requires = vec![&[] as &[&str], &["a"][..], &["b"][..]];
    let order = ConfigLoaderFactory::topology_sort(&names, &requires)
        .expect("topology");
    assert_eq!(order.len(), 3);
    assert!(order[0] < order[1] && order[1] < order[2]);
}

#[test]
fn test_create_config_builder_happy() {
    let builder = ConfigLoaderFactory::create_config_builder();
    assert!(!builder.name.is_empty());
    assert!(!builder.version.is_empty());
}
