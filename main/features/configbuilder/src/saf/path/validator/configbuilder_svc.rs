use crate::api::error::config_error::ConfigError;
use crate::api::traits::validator::Validator;
use crate::core::DefaultValidator;

pub(crate) struct PathValidatorImpl;

impl Validator for PathValidatorImpl {
    fn validate_path(&self, target: &std::path::Path) -> Result<(), ConfigError> {
        DefaultValidator.validate_path(target)
    }
}
