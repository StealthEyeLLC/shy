//! TextSHY domain
//!
//! PUBLIC API FROZEN — changes require version bump

use core::{Domain, DomainId};

/// Canonical TextSHY domain type
#[derive(Debug, Default)]
pub struct TextDomain;

impl TextDomain {
    pub fn new() -> Self {
        TextDomain
    }

    /// Accepts text input and returns a stable handle.
    ///
    /// Behavior intentionally undefined in PASS 2A.
    pub fn ingest_text(&self, _text: &str) -> Result<TextHandle, TextError> {
        todo!()
    }
}

impl Domain for TextDomain {
    fn id(&self) -> DomainId {
        DomainId("textshy".to_string())
    }
}

/// Public handle type (API surface only)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextHandle {
    pub id: String,
}

/// Public error type (API surface only)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextError {
    InvalidInput,
    Rejected,
}
