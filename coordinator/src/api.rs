// PUBLIC API FROZEN — changes require version bump
use crate::{CoordinatorError, FlowState};
use core::{Domain, Request};
use whitehat::{Decision, Policy};

pub trait Coordinator {
    fn handle(
        &self,
        policy: &dyn Policy,
        domain: &dyn Domain,
        request: &dyn Request,
    ) -> Result<FlowState, CoordinatorError>;
}

pub fn run(
    coordinator: &dyn Coordinator,
    policy: &dyn Policy,
    domain: &dyn Domain,
    request: &dyn Request,
) -> Result<FlowState, CoordinatorError> {
    coordinator.handle(policy, domain, request)
}
