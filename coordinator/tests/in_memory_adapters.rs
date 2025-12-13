use coordinator::{
    InMemoryAuditSink, InMemoryFlowStore, AuditEvent, FlowState, AuditSink, FlowStore,
};
use core::{DomainId, RequestId};

#[test]
fn in_memory_flow_store_records() {
    let store = InMemoryFlowStore::new();
    let domain = DomainId("d".into());
    let request = RequestId("r".into());

    store.record_flow(&domain, &request, FlowState::Completed);
    assert_eq!(store.records_len(), 1);
}

#[test]
fn in_memory_audit_sink_records() {
    let sink = InMemoryAuditSink::new();
    let domain = DomainId("d".into());
    let request = RequestId("r".into());

    sink.emit(&domain, &request, AuditEvent::FlowStarted);
    assert_eq!(sink.events_len(), 1);
}
