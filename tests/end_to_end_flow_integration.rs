use coordinator::{EndToEndFlow, Enforcement, FlowState};
use core::{Domain, DomainId, Request, RequestId};
use whitehat::{Decision, Policy};
use cli::{Cli, ExitCode};

struct TestDomain;
impl Domain for TestDomain {
    fn id(&self) -> DomainId {
        DomainId("domain".into())
    }
}

struct TestRequest;
impl Request for TestRequest {
    fn id(&self) -> RequestId {
        RequestId("request".into())
    }
}

struct AllowPolicy;
impl Policy for AllowPolicy {
    fn evaluate(&self, _input: &str) -> Result<Decision, whitehat::WhiteHatError> {
        Ok(Decision::Allow)
    }
}

struct DenyPolicy;
impl Policy for DenyPolicy {
    fn evaluate(&self, _input: &str) -> Result<Decision, whitehat::WhiteHatError> {
        Ok(Decision::Deny)
    }
}

#[test]
fn allow_policy_completes_flow() {
    let enforcement = Enforcement::new();
    let flow = EndToEndFlow::new();
    let domain = TestDomain;
    let request = TestRequest;
    let policy = AllowPolicy;

    let result = flow.run(&enforcement, &policy, &domain, &request).unwrap();
    assert_eq!(result, FlowState::Completed);

    let cli = Cli;
    let cli_result = cli.run(&[]);
    assert_eq!(cli_result.unwrap(), ExitCode::Success);
}

#[test]
fn deny_policy_rejects_flow() {
    let enforcement = Enforcement::new();
    let flow = EndToEndFlow::new();
    let domain = TestDomain;
    let request = TestRequest;
    let policy = DenyPolicy;

    let result = flow.run(&enforcement, &policy, &domain, &request).unwrap();
    assert_eq!(result, FlowState::Rejected);
}
