//! Coordinator ↔ HBR Bridge (v0)
//!
//! Purpose:
//! - Run the HBR approval gate over a VFY result.
//! - No execution authority. Explicit call only.

use hbr::{Gate, HbrOutcome};
use vfy::VfyResult;

pub fn decide(vfy: &VfyResult) -> HbrOutcome {
    Gate::decide(vfy)
}
