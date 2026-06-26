/// Error type for environment variable substitution failures.
#[derive(Debug, Clone)]
pub enum SubstitutionError {
    VariableNotFound {
        var_name: String,
        location: String,
    },
    VariableRejected {
        var_name: String,
        reason: String,
        policy: String,
    },
    InvalidSubstitutionSyntax {
        value: String,
        reason: String,
    },
}
