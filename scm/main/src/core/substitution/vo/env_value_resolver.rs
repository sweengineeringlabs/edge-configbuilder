use crate::api::SubstitutionError;
use crate::{EnvValueResolver, ValueResolver};

impl ValueResolver for EnvValueResolver {
    fn resolve(&self, var_name: &str, location: &str) -> Result<String, SubstitutionError> {
        std::env::var(var_name).map_err(|_| SubstitutionError::VariableNotFound {
            var_name: var_name.to_string(),
            location: location.to_string(),
        })
    }
}

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    use super::*;

    fn must<T, E>(result: Result<T, E>) -> T {
        result.unwrap_or_else(|_| std::process::abort())
    }

    #[test]
    fn test_resolve_existing_env_var_returns_value() {
        // SAFETY: test-only; no concurrent env access in this test binary at this point.
        unsafe { std::env::set_var("CONFIGBUILDER_TEST_RESOLVE_VAR", "value123") };
        let result = EnvValueResolver.resolve("CONFIGBUILDER_TEST_RESOLVE_VAR", "loc");
        // SAFETY: cleanup
        unsafe { std::env::remove_var("CONFIGBUILDER_TEST_RESOLVE_VAR") };
        assert_eq!(must(result), "value123");
    }

    #[test]
    fn test_resolve_missing_env_var_returns_variable_not_found_with_location() {
        let result = EnvValueResolver.resolve("CONFIGBUILDER_TEST_ABSENT_XYZ", "app.toml:key");
        assert_eq!(
            result,
            Err(SubstitutionError::VariableNotFound {
                var_name: "CONFIGBUILDER_TEST_ABSENT_XYZ".to_string(),
                location: "app.toml:key".to_string(),
            })
        );
    }
}
