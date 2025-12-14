/// Coordinator-level errors
#[derive(Debug)]
pub enum CoordinatorError {
    InvalidPolicy,
    ExecutionFailed,
}

/// Public flow outcome state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowState {
    Initialized,
    Completed,
    Rejected,
}
