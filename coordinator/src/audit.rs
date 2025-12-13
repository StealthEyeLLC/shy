// PASS 1 — structural only

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditEvent {
    FlowStarted,
    PolicyEvaluated,
    FlowCompleted,
    FlowRejected,
}
