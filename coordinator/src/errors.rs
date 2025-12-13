#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoordinatorError {
    GovernanceDenied,
    InvalidRequest,
}
