use coordinator::Enforcement;
use core::{Domain, DomainId, Request, RequestId};
use whitehat::{Decision, Policy};

struct TestDomain;
impl Domain for TestDomain {
    fn id(&self) -> DomainId {
        DomainId("domain".into())
    }
}

struct TestRequest;
impl Request for TestRequest {
    fn id(&self) -> RequestId {
        RequestId("req".into())
    }
}

struct DenyPolicy;
impl Policy for DenyPolicy {
    fn evaluate(&self, _input: &str) -> Result<Decision, whitehat::WhiteHatError> {
        Ok(Decision::Deny)
    }
}

struct AllowPolicy;
impl Policy for AllowPolicy {
    fn evaluate(&self, _input: &str) -> Result<Decision, whitehat::WhiteHatError> {
        Ok(Decision::Allow)
    }
}

#[test]
fn deny_policy_blocks_flow() {
    let enforcement = Enforcement::new();
    let policy = DenyPolicy;
    let domain = TestDomain;
    let request = TestRequest;

    let decision = enforcement.evaluate(&policy, &domain, &request).unwrap();
    assert_eq!(decision, Decision::Deny);
}

#[test]
fn allow_policy_permits_flow() {
    let enforcement = Enforcement::new();
    let policy = AllowPolicy;
    let domain = TestDomain;
    let request = TestRequest;

    let decision = enforcement.evaluate(&policy, &domain, &request).unwrap();
    assert_eq!(decision, Decision::Allow);
}
