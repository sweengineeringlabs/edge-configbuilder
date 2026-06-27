/// Error type for environment variable substitution failures.
#[derive(Debug, Clone, PartialEq)]
pub enum SubstitutionError {
    /// A referenced environment variable does not exist in the process environment.
    VariableNotFound {
        /// Name of the missing variable.
        var_name: String,
        /// Config file or section where the placeholder appeared.
        location: String,
    },
    /// A variable was found but rejected by the active substitution policy.
    VariableRejected {
        /// Name of the rejected variable.
        var_name: String,
        /// Human-readable reason the policy rejected the variable.
        reason: String,
        /// Description of the policy that rejected the variable.
        policy: String,
    },
    /// The substitution syntax in the config value is malformed.
    InvalidSubstitutionSyntax {
        /// The raw config value containing the malformed placeholder.
        value: String,
        /// Description of the syntax error.
        reason: String,
    },
}
