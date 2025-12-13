/// PUBLIC API FROZEN — changes require version bump
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusalReason {
    PolicyDenied,
    InvalidInput,
    Unauthorized,
}
