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
    /// Deterministic behavior:
    /// - Empty or whitespace-only input is rejected
    /// - Non-empty input yields a stable handle
    pub fn ingest_text(&self, text: &str) -> Result<TextHandle, TextError> {
        if text.trim().is_empty() {
            Err(TextError::InvalidInput)
        } else {
            Ok(TextHandle {
                id: "textshy:handle".to_string(),
            })
        }
    }
}

impl Domain for TextDomain {
    fn id(&self) -> DomainId {
        DomainId("textshy".to_string())
    }
}

/// Public handle type (API surface)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextHandle {
    pub id: String,
}

/// Public error type (API surface)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextError {
    InvalidInput,
    Rejected,
}
