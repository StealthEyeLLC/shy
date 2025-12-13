//! TextSHY domain
//!
//! PUBLIC API FROZEN — changes require version bump

use core::{Domain, DomainId};

#[derive(Debug, Default)]
pub struct TextDomain;

impl TextDomain {
    pub fn new() -> Self {
        TextDomain
    }
}

impl Domain for TextDomain {
    fn id(&self) -> DomainId {
        DomainId("textshy".to_string())
    }
}
