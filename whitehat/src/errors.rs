#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhiteHatError {
    PolicyViolation,
    Unauthorized,
    InvalidRequest,
}
