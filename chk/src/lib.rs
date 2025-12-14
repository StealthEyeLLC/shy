//! CHK v0 — Invariant Checking SSPU
//!
//! Purpose (v0):
//! - Check system invariants after cleanup resolution.
//! - NO execution authority.
//! - NO mutation.
//! - Deterministic, fact-based checks only.
//!
//! Inputs: CLR cleanup results
//! Outputs: Invariant check report

use clr::{CleanupAction, CleanupResult};

#[derive(Clone, Debug)]
pub struct InvariantViolation {
    pub code: &'static str,
    pub detail: String,
}

#[derive(Clone, Debug)]
pub struct InvariantReport {
    pub ok: bool,
    pub violations: Vec<InvariantViolation>,
}

impl InvariantReport {
    pub fn ok() -> Self {
        Self {
            ok: true,
            violations: Vec::new(),
        }
    }

    pub fn fail(mut self, code: &'static str, detail: impl Into<String>) -> Self {
        self.ok = false;
        self.violations.push(InvariantViolation {
            code,
            detail: detail.into(),
        });
        self
    }
}

/// Deterministic invariant checker.
/// v0 rules:
/// - If cleanup requested ResetLocalState, invariant requires that at least one action exists
/// - No other assumptions
pub struct Checker;

impl Checker {
    pub fn check(result: &CleanupResult) -> InvariantReport {
        let mut report = InvariantReport::ok();

        if result.actions.contains(&CleanupAction::ResetLocalState) && result.actions.is_empty() {
            report = report.fail(
                "CHK_INVARIANT_RESET_MISSING",
                "ResetLocalState required but no cleanup actions present",
            );
        }

        report
    }
}
