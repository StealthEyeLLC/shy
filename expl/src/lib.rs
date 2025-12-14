//! EXPL v0 — Explanation SSPU
//!
//! Purpose (v0):
//! - Produce human-readable, fact-based explanations of what occurred.
//! - NO authority.
//! - NO execution.
//! - NO speculation.
//! - Deterministic rendering only.

use obs::ObsEvent;
use vfy::VfyResult;
use hbr::{HbrDecision, HbrOutcome};
use wdn::{EnforcementResult, EnforcementState};
use clr::{CleanupAction, CleanupResult};
use chk::InvariantReport;

/// Render a deterministic, human-readable explanation.
pub fn explain(
    obs: &[ObsEvent],
    vfy: &VfyResult,
    hbr: &HbrOutcome,
    wdn: &EnforcementResult,
    clr: &CleanupResult,
    chk: &InvariantReport,
) -> String {
    let mut out = String::new();

    out.push_str("=== EXECUTION EXPLANATION ===\n\n");

    out.push_str("OBS — Observed Events:\n");
    if obs.is_empty() {
        out.push_str("  (no events recorded)\n");
    } else {
        for (i, e) in obs.iter().enumerate() {
            out.push_str(&format!(
                "  [{}] {} | {} | {}\n",
                i, e.timestamp_ms, e.source, e.message
            ));
        }
    }

    out.push_str("\nVFY — Verification:\n");
    if vfy.ok {
        out.push_str("  Verification passed.\n");
    } else {
        out.push_str("  Verification failed:\n");
        for issue in &vfy.issues {
            out.push_str(&format!(
                "    - {}: {}\n",
                issue.code, issue.detail
            ));
        }
    }

    out.push_str("\nHBR — Approval Gate:\n");
    match hbr.decision {
        HbrDecision::Approve => {
            out.push_str("  Decision: APPROVED\n");
        }
        HbrDecision::Deny => {
            out.push_str("  Decision: DENIED\n");
            for r in &hbr.reasons {
                out.push_str(&format!("    - {}\n", r));
            }
        }
    }

    out.push_str("\nWDN — Enforcement:\n");
    match wdn.state {
        EnforcementState::Allow => {
            out.push_str("  State: ALLOW\n");
        }
        EnforcementState::Contained => {
            out.push_str("  State: CONTAINED\n");
            for r in &wdn.reasons {
                out.push_str(&format!("    - {}\n", r));
            }
        }
    }

    out.push_str("\nCLR — Cleanup:\n");
    if clr.actions.is_empty() {
        out.push_str("  No cleanup actions.\n");
    } else {
        for a in &clr.actions {
            match a {
                CleanupAction::ResetLocalState => {
                    out.push_str("  Action: ResetLocalState\n");
                }
                CleanupAction::None => {
                    out.push_str("  Action: None\n");
                }
            }
        }
        for n in &clr.notes {
            out.push_str(&format!("    Note: {}\n", n));
        }
    }

    out.push_str("\nCHK — Invariants:\n");
    if chk.ok {
        out.push_str("  All invariants satisfied.\n");
    } else {
        out.push_str("  Invariant violations:\n");
        for v in &chk.violations {
            out.push_str(&format!(
                "    - {}: {}\n",
                v.code, v.detail
            ));
        }
    }

    out.push_str("\n=== END EXPLANATION ===\n");

    out
}
