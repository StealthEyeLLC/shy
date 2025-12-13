#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    InvalidInput,
    Unauthorized,
    NotFound,
}
