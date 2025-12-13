#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowState {
    Initialized,
    Completed,
    Rejected,
}
