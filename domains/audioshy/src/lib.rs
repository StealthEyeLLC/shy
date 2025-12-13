//! AudioSHY domain
//!
//! PUBLIC API FROZEN — changes require version bump

use core::{Domain, DomainId};

/// Canonical AudioSHY domain marker
#[derive(Debug, Default)]
pub struct AudioDomain;

impl AudioDomain {
    pub fn new() -> Self {
        AudioDomain
    }
}

impl Domain for AudioDomain {
    fn id(&self) -> DomainId {
        DomainId("audioshy".to_string())
    }
}
