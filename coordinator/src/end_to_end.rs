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
        // Governance-first: enforce before any capability execution.
        let decision = enforcement
            .evaluate(policy, domain, request)
            .map_err(|_| CoordinatorError::InvalidRequest)?;

        self.map_decision(decision)
    }

    pub fn map_decision(&self, decision: Decision) -> Result<FlowState, CoordinatorError> {
        match decision {
            Decision::Allow => Ok(FlowState::Completed),
            Decision::Deny => Ok(FlowState::Rejected),
        }
    }
}
