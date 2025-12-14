//! CLR v0 — Cleanup / Hygiene / Recovery SSPU
//!
//! Purpose (v0):
//! - Perform explicit cleanup signaling after enforcement.
//! - NO execution authority.
//! - NO retries.
//! - NO mutation of upstream state.
//! - Deterministic, side-effect bounded.

use wdn::{EnforcementResult, EnforcementState};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CleanupAction {
    None,
    ResetLocalState,
}

#[derive(Clone, Debug)]
pub struct CleanupResult {
    pub actions: Vec<CleanupAction>,
    pub notes: Vec<String>,
}

impl CleanupResult {
    pub fn none() -> Self {
        Self {
            actions: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn with_action(action: CleanupAction, note: impl Into<String>) -> Self {
        Self {
            actions: vec![action],
            notes: vec![note.into()],
        }
    }
}

/// Deterministic cleanup resolver.
pub struct Cleaner;

impl Cleaner {
    pub fn resolve(enforcement: &EnforcementResult) -> CleanupResult {
        match enforcement.state {
            EnforcementState::Allow => CleanupResult::none(),
            EnforcementState::Contained => CleanupResult::with_action(
                CleanupAction::ResetLocalState,
                "Execution contained; local state reset suggested",
            ),
        }
    }
}
