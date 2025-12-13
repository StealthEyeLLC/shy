/// PUBLIC API FROZEN — changes require version bump
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditEvent {
    FlowStarted,
    PolicyEvaluated,
    FlowCompleted,
    FlowRejected,
}
