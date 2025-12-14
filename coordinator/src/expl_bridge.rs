//! Coordinator ↔ EXPL Bridge (v0)
//!
//! Purpose:
//! - Produce a human-readable explanation from system artifacts.
//! - NO execution authority.
//! - Explicit, deterministic call only.

use chk::InvariantReport;
use clr::CleanupResult;
use expl::explain;
use hbr::HbrOutcome;
use obs::ObsEvent;
use vfy::VfyResult;
use wdn::EnforcementResult;

pub fn explain_all(
    obs: &[ObsEvent],
    vfy: &VfyResult,
    hbr: &HbrOutcome,
    wdn: &EnforcementResult,
    clr: &CleanupResult,
    chk: &InvariantReport,
) -> String {
    explain(obs, vfy, hbr, wdn, clr, chk)
}
