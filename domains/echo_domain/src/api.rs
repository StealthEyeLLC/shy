use core::Request;
use whitehat::Decision;

/// PUBLIC API FROZEN — changes require version bump
pub struct EchoDomain;

impl EchoDomain {
    pub fn new() -> Self {
        EchoDomain
    }

    pub fn handle(&self, _request: &dyn Request) -> Decision {
        Decision::Allow
    }
}
