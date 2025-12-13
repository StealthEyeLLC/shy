use core::{Domain, DomainId};

/// PUBLIC API FROZEN — changes require version bump
pub struct PhotoShyDomain;

impl PhotoShyDomain {
    pub fn new() -> Self {
        PhotoShyDomain
    }

    /// Accepts a photo input and returns a stable handle.
    ///
    /// Behavior intentionally undefined in PASS 2A.
    pub fn ingest_photo(&self, _bytes: &[u8]) -> Result<PhotoHandle, PhotoError> {
        todo!()
    }
}

/// PUBLIC API FROZEN — changes require version bump
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhotoHandle {
    pub id: String,
}

/// PUBLIC API FROZEN — changes require version bump
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PhotoError {
    InvalidInput,
    Rejected,
}

impl Domain for PhotoShyDomain {
    fn id(&self) -> DomainId {
        DomainId("photoshy".into())
    }
}
