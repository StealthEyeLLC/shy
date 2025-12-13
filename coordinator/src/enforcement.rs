use core::{Domain, Request};
use whitehat::{Decision, Policy};

/// PUBLIC API FROZEN — changes require version bump
pub struct Enforcement;

impl Enforcement {
    pub fn new() -> Self {
        Enforcement
    }

    pub fn evaluate(
        &self,
        policy: &dyn Policy,
        _domain: &dyn Domain,
        request: &dyn Request,
    ) -> Result<Decision, ()> {
        // Deterministic input: request id string
        match policy.evaluate(&request.id().0) {
            Ok(decision) => Ok(decision),
            Err(_) => Err(()),
        }
    }
}
