#[cfg(any(test, feature = "test-utils"))]
use crate::api::SubstitutionError;
#[cfg(any(test, feature = "test-utils"))]
use crate::{AllowAllPolicy, SubstitutionPolicy};

#[cfg(any(test, feature = "test-utils"))]
impl SubstitutionPolicy for AllowAllPolicy {
    fn validate(&self, _var_name: &str) -> Result<(), SubstitutionError> {
        Ok(())
    }

    fn description(&self) -> String {
        "AllowAll (no restrictions)".to_string()
    }
}
