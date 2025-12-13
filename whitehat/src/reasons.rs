// PASS 1 — structural only

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusalReason {
    PolicyDenied,
    InvalidInput,
    Unauthorized,
}
