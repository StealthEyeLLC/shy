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
        domain: &dyn Domain,
        request: &dyn Request,
    ) -> Result<Decision, ()> {
        let _ = (policy, domain, request);
        todo!()
    }
}
