use coordinator::{
    EndToEndFlow, Enforcement, FlowState,
    InMemoryFlowStore, InMemoryAuditSink,
    AuditEvent,
};
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
        RequestId("request".into())
    }
}

struct AllowPolicy;
impl Policy for AllowPolicy {
    fn evaluate(&self, _input: &str) -> Result<Decision, whitehat::WhiteHatError> {
        Ok(Decision::Allow)
    }
}

#[test]
fn end_to_end_flow_can_record_state_and_audit() {
    let enforcement = Enforcement::new();
    let flow = EndToEndFlow::new();

    let store = InMemoryFlowStore::new();
    let audit = InMemoryAuditSink::new();

    let domain = TestDomain;
    let request = TestRequest;
    let policy = AllowPolicy;

    let result = flow.run(&enforcement, &policy, &domain, &request).unwrap();
    assert_eq!(result, FlowState::Completed);

    // simulate wiring (no automatic recording yet by design)
    store.record_flow(&domain.id(), &request.id(), result);
    audit.emit(&domain.id(), &request.id(), AuditEvent::FlowCompleted);

    assert_eq!(store.records_len(), 1);
    assert_eq!(audit.events_len(), 1);
}
