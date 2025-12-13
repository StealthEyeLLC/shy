use core::{Domain, DomainId};

/// PUBLIC API FROZEN — changes require version bump
pub struct PhotoShyDomain;

impl PhotoShyDomain {
    pub fn new() -> Self {
        PhotoShyDomain
    }
}

impl Domain for PhotoShyDomain {
    fn id(&self) -> DomainId {
        DomainId("photoshy".into())
    }
}
