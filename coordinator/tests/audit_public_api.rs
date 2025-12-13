use coordinator::AuditEvent;

#[test]
fn audit_event_is_constructible() {
    let _ = AuditEvent::FlowStarted;
}
