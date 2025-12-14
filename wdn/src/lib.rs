//! WDN v0 — Watchdog / Enforcement / Containment SSPU
//!
//! Purpose (v0):
//! - Enforce explicit allow/deny outcomes.
//! - Provide containment signaling only.
//! - NO execution authority.
//! - NO retries, NO escalation, NO intelligence.
//!
//! Inputs: HBR outcomes
//! Outputs: Enforcement state (Allow | Contained)

use hbr::{HbrDecision, HbrOutcome};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnforcementState {
    Allow,
    Contained,
}

#[derive(Clone, Debug)]
pub struct EnforcementResult {
    pub state: EnforcementState,
    pub reasons: Vec<String>,
}

impl EnforcementResult {
    pub fn allow() -> Self {
        Self {
            state: EnforcementState::Allow,
            reasons: Vec::new(),
        }
    }

    pub fn contained(reasons: Vec<String>) -> Self {
        Self {
            state: EnforcementState::Contained,
            reasons,
        }
    }
}

/// Deterministic enforcement boundary.
/// v0 rule:
/// - Allow iff HBR approved
/// - Otherwise contain, with reasons
pub struct Enforcer;

impl Enforcer {
    pub fn enforce(outcome: &HbrOutcome) -> EnforcementResult {
        match outcome.decision {
            HbrDecision::Approve => EnforcementResult::allow(),
            HbrDecision::Deny => EnforcementResult::contained(outcome.reasons.clone()),
        }
    }
}
