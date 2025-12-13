use crate::{CoordinatorError, Enforcement, FlowState, AuditEvent};
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
        // Audit: flow started
        let _event = AuditEvent::FlowStarted;

        // Governance-first enforcement
        let decision = enforcement
            .evaluate(policy, domain, request)
            .map_err(|_| CoordinatorError::InvalidRequest)?;

        // Audit: policy evaluated
        let _event = AuditEvent::PolicyEvaluated;

        self.map_decision(decision)
    }

    pub fn map_decision(&self, decision: Decision) -> Result<FlowState, CoordinatorError> {
        match decision {
            Decision::Allow => {
                let _event = AuditEvent::FlowCompleted;
                Ok(FlowState::Completed)
            }
            Decision::Deny => {
                let _event = AuditEvent::FlowRejected;
                Ok(FlowState::Rejected)
            }
        }
    }
}
