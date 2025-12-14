use crate::{Decision, WhiteHatError};

/// PUBLIC API FROZEN — changes require version bump
pub trait Policy {
    fn evaluate(&self, input: &str) -> Result<Decision, WhiteHatError>;
}
