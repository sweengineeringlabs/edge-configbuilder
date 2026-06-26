/// Allows only environment variables whose names start with one of the given prefixes.
#[derive(Debug)]
pub struct PrefixWhitelistPolicy {
    pub(crate) prefixes: Vec<String>,
}
