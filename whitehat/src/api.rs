// PUBLIC API FROZEN — changes require version bump
use crate::{Decision, Reason, WhiteHatError};

pub trait Policy {
    fn evaluate(&self, input: &str) -> Result<Decision, WhiteHatError>;
}

pub fn enforce(
    policy: &dyn Policy,
    input: &str,
) -> Result<Decision, WhiteHatError> {
    policy.evaluate(input)
}
