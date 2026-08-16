/// Allows only environment variables whose names fully match a regular expression.
#[derive(Debug)]
pub struct PatternWhitelistPolicy {
    pub(crate) pattern: regex::Regex,
    pub(crate) pattern_str: String,
}
