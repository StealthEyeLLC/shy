//! Coordinator ↔ WDN Bridge (v0)
//!
//! Purpose:
//! - Apply enforcement over HBR outcomes.
//! - No execution authority.
//! - Explicit, deterministic call only.

use hbr::HbrOutcome;
use wdn::{EnforcementResult, Enforcer};

pub fn enforce(outcome: &HbrOutcome) -> EnforcementResult {
    Enforcer::enforce(outcome)
}
//! Coordinator ↔ WDN Bridge (v0)
//!
//! Purpose:
//! - Apply enforcement over HBR outcomes.
//! - No execution authority.
//! - Explicit, deterministic call only.

use hbr::HbrOutcome;
use wdn::{EnforcementResult, Enforcer};

pub fn enforce(outcome: &HbrOutcome) -> EnforcementResult {
    Enforcer::enforce(outcome)
}
