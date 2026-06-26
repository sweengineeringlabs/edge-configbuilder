use crate::{PrefixWhitelistPolicy, SubstitutionPolicy};

impl SubstitutionPolicy for PrefixWhitelistPolicy {
    fn validate(&self, var_name: &str) -> Result<(), String> {
        if self
            .prefixes
            .iter()
            .any(|p| var_name.starts_with(p.as_str()))
        {
            Ok(())
        } else {
            Err(format!(
                "Variable '{}' does not match any allowed prefix: {}",
                var_name,
                self.prefixes.join(", ")
            ))
        }
    }

    fn description(&self) -> String {
        format!("PrefixWhitelist({})", self.prefixes.join(", "))
    }
}
