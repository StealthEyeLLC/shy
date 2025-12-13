use crate::{Decision, Policy, WhiteHatError};

/// PUBLIC API FROZEN — changes require version bump
pub struct AllowAllPolicy;

impl AllowAllPolicy {
    pub fn new() -> Self {
        AllowAllPolicy
    }
}

impl Policy for AllowAllPolicy {
    fn evaluate(&self, _input: &str) -> Result<Decision, WhiteHatError> {
        Ok(Decision::Allow)
    }
}
