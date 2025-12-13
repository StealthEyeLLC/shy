//! PhotoSHY domain
//!
//! PUBLIC API FROZEN — changes require version bump

use core::{Domain, DomainId};

/// Canonical PhotoSHY domain type
///
/// NOTE:
/// - Domain names are *generic* (PhotoDomain),
///   not branded (PhotoShyDomain),
///   for ecosystem consistency.
#[derive(Debug, Default)]
pub struct PhotoDomain;

impl PhotoDomain {
    pub fn new() -> Self {
        PhotoDomain
    }
}

impl Domain for PhotoDomain {
    fn id(&self) -> DomainId {
        DomainId("photoshy".to_string())
    }
}
