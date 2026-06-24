const NOT_A_DIR_MSG: &str = "config path exists but is not a directory";
use crate::api::{Validator, ValidatorBound, ValidatorError, ValidatorOps};
use std::path::Path;

pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate_path(&self, target: &Path) -> Result<(), ValidatorError> {
        if target.exists() && !target.is_dir() {
            return Err(ValidatorError::Io(format!(
                "{}: {NOT_A_DIR_MSG}",
                target.display()
            )));
        }
        Ok(())
    }
}

impl ValidatorOps for DefaultValidator {
    fn check_path(&self, target: &Path) -> Result<(), ValidatorError> {
        <Self as Validator>::validate_path(self, target)
    }
}

impl ValidatorBound for DefaultValidator {
    type PathValidator = crate::api::PathValidatorImpl;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn must<T, E>(result: Result<T, E>) -> T {
        result.unwrap_or_else(|_| std::process::abort())
    }

    fn must_err<T, E>(result: Result<T, E>) -> E {
        match result {
            Ok(_) => std::process::abort(),
            Err(err) => err,
        }
    }

    #[test]
    fn test_validate_path_accepts_nonexistent_path() {
        let path = Path::new("/nonexistent/swe-edge-test-xyz");
        assert!(!path.exists(), "test path must remain absent");
        assert!(matches!(DefaultValidator.validate_path(path), Ok(())));
    }

    #[test]
    fn test_validate_path_accepts_existing_dir() {
        let dir = must(tempfile::tempdir());
        assert!(dir.path().is_dir(), "tempdir must create a directory");
        assert!(DefaultValidator.validate_path(dir.path()).is_ok());
    }

    #[test]
    fn test_validate_path_rejects_file() {
        let dir = must(tempfile::tempdir());
        let file = dir.path().join("not_a_dir.toml");
        must(std::fs::write(&file, b""));
        let err = must_err(DefaultValidator.validate_path(&file));
        assert!(matches!(err, ValidatorError::Io(_)));
        assert!(err.to_string().contains("not a directory"));
    }
}
