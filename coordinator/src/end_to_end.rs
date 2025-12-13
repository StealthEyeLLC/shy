use crate::{CoordinatorError, Enforcement, FlowState};
use core::{Domain, Request};
use whitehat::{Decision, Policy};

/// PUBLIC API FROZEN — changes require version bump
pub struct EndToEndFlow;

impl EndToEndFlow {
    pub fn new() -> Self {
        EndToEndFlow
    }

    pub fn run(
        &self,
        enforcement: &Enforcement,
        policy: &dyn Policy,
        domain: &dyn Domain,
        request: &dyn Request,
    ) -> Result<FlowState, CoordinatorError> {
        let _ = (enforcement, policy, domain, request);
        todo!()
    }

    pub fn map_decision(&self, decision: Decision) -> Result<FlowState, CoordinatorError> {
        let _ = decision;
        todo!()
    }
}
