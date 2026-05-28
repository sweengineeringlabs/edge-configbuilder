const NOT_A_DIR_MSG: &str = "config path exists but is not a directory";
use crate::api::error::config_error::ConfigError;
use crate::api::traits::validator::Validator;
use crate::api::traits::validator_ops::ValidatorOps;
use std::path::Path;

pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate_path(&self, target: &Path) -> Result<(), ConfigError> {
        if target.exists() && !target.is_dir() {
            return Err(ConfigError::Io(format!(
                "{}: {NOT_A_DIR_MSG}",
                target.display()
            )));
        }
        Ok(())
    }
}

impl ValidatorOps for DefaultValidator {
    fn check_path(&self, target: &Path) -> Result<(), ConfigError> {
        <Self as Validator>::validate_path(self, target)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_path_accepts_nonexistent_path() {
        assert!(DefaultValidator
            .validate_path(Path::new("/nonexistent/swe-edge-test-xyz"))
            .is_ok());
    }

    #[test]
    fn test_validate_path_accepts_existing_dir() {
        let dir = tempfile::tempdir().unwrap();
        assert!(DefaultValidator.validate_path(dir.path()).is_ok());
    }

    #[test]
    fn test_validate_path_rejects_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("not_a_dir.toml");
        std::fs::write(&file, b"").unwrap();
        let err = DefaultValidator.validate_path(&file).unwrap_err();
        assert!(matches!(err, ConfigError::Io(_)));
        assert!(err.to_string().contains("not a directory"));
    }
}
