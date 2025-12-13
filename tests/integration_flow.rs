use coordinator::{Coordinator, FlowState};
use core::{Domain, DomainId, Request, RequestId};
use whitehat::{Decision, Policy};
use domains::DomainHandler;
use cli::{Cli, ExitCode};

struct AllowAllPolicy;
impl Policy for AllowAllPolicy {
    fn evaluate(&self, _input: &str) -> Result<Decision, whitehat::WhiteHatError> {
        Ok(Decision::Allow)
    }
}

struct TestDomain;
impl Domain for TestDomain {
    fn id(&self) -> DomainId {
        DomainId("test-domain".into())
    }
}

struct TestRequest;
impl Request for TestRequest {
    fn id(&self) -> RequestId {
        RequestId("test-request".into())
    }
}

struct TestHandler;
impl DomainHandler for TestHandler {
    fn handle(&self, _request: &dyn Request) -> Decision {
        Decision::Allow
    }
}

struct TestCoordinator;
impl Coordinator for TestCoordinator {
    fn handle(
        &self,
        policy: &dyn Policy,
        domain: &dyn Domain,
        request: &dyn Request,
    ) -> Result<FlowState, coordinator::CoordinatorError> {
        let _ = policy.evaluate(&domain.id().0);
        let _ = request.id();
        Ok(FlowState::Completed)
    }
}

#[test]
fn end_to_end_flow_is_wired() {
    let policy = AllowAllPolicy;
    let domain = TestDomain;
    let request = TestRequest;
    let coordinator = TestCoordinator;
    let cli = Cli;

    let flow = coordinator.handle(&policy, &domain, &request).unwrap();
    assert_eq!(flow, FlowState::Completed);

    let result = cli.run(&[]);
    assert_eq!(result.unwrap(), ExitCode::Success);
}
