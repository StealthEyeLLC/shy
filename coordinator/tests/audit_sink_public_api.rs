use coordinator::{AuditSink, AuditEvent};
use core::{DomainId, RequestId};

struct TestSink;

impl AuditSink for TestSink {
    fn emit(
        &self,
        _domain: &DomainId,
        _request: &RequestId,
        _event: AuditEvent,
    ) {}
}

#[test]
fn audit_sink_trait_is_implementable() {
    let _sink = TestSink;
}
