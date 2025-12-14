//! HBR v0 — Harbinger (Foresight / Zero Trust / Approval)
//!
//! Purpose (v0):
//! - Provide an explicit, deterministic approval gate over verification results.
//! - No execution authority.
//! - No prediction, no speculation, no autonomy.
//!
//! Inputs: VFY results (proof outputs)
//! Outputs: Approve / Deny decision + reasons

use vfy::VfyResult;

#[derive(Clone, Debug)]
pub enum HbrDecision {
    Approve,
    Deny,
}

#[derive(Clone, Debug)]
pub struct HbrOutcome {
    pub decision: HbrDecision,
    pub reasons: Vec<String>,
}

impl HbrOutcome {
    pub fn approve() -> Self {
        Self {
            decision: HbrDecision::Approve,
            reasons: Vec::new(),
        }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            decision: HbrDecision::Deny,
            reasons: vec![reason.into()],
        }
    }

    pub fn deny_many(reasons: Vec<String>) -> Self {
        Self {
            decision: HbrDecision::Deny,
            reasons,
        }
    }
}

/// Deterministic approval gate.
/// v0 rule:
/// - Approve iff VFY says ok
/// - Deny otherwise with explicit reasons
pub struct Gate;

impl Gate {
    pub fn decide(vfy: &VfyResult) -> HbrOutcome {
        if vfy.ok {
            return HbrOutcome::approve();
        }

        let mut reasons: Vec<String> = Vec::new();
        for issue in &vfy.issues {
            reasons.push(format!("{}: {}", issue.code, issue.detail));
        }

        if reasons.is_empty() {
            reasons.push("VFY failed with no issues (unexpected)".to_string());
        }

        HbrOutcome::deny_many(reasons)
    }
}
