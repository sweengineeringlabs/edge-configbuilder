use crate::{PathValidatorImpl, ValidatorError};

impl crate::Validator for PathValidatorImpl {
    fn validate_path(&self, target: &std::path::Path) -> Result<(), ValidatorError> {
        self.ops.check_path(target)
    }
}
