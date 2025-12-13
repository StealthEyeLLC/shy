use coordinator::{Coordinator, FlowState};
use core::{Domain, DomainId, Request, RequestId};
use whitehat::{Decision, Policy};

struct TestCoordinator;

impl Coordinator for TestCoordinator {
    fn handle(
        &self,
        _policy: &dyn Policy,
        _domain: &dyn Domain,
        _request: &dyn Request,
    ) -> Result<FlowState, coordinator::CoordinatorError> {
        Ok(FlowState::Initialized)
    }
}

struct TestDomain;
impl Domain for TestDomain {
    fn id(&self) -> DomainId {
        DomainId("d".into())
    }
}

struct TestRequest;
impl Request for TestRequest {
    fn id(&self) -> RequestId {
        RequestId("r".into())
    }
}

struct AllowAll;
impl Policy for AllowAll {
    fn evaluate(&self, _input: &str) -> Result<Decision, whitehat::WhiteHatError> {
        Ok(Decision::Allow)
    }
}

#[test]
fn coordinator_is_callable() {
    let c = TestCoordinator;
    let p = AllowAll;
    let d = TestDomain;
    let r = TestRequest;
    let _ = c.handle(&p, &d, &r);
}
