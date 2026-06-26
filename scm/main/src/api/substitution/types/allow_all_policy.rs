#[cfg(any(test, feature = "test-utils"))]
#[derive(Debug)]
/// Test-only policy that accepts every variable.
pub struct AllowAllPolicy;
