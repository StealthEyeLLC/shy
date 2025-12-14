//! Coordinator ↔ VFY Bridge (v0)
//!
//! Purpose:
//! - Provide a minimal helper that runs VFY over an OBS snapshot.
//! - No execution authority is granted to VFY.
//! - Deterministic, explicit call only.

use obs::ObsEvent;
use vfy::{Verifier, VfyResult};

pub fn verify_obs_snapshot(events: &[ObsEvent]) -> VfyResult {
    Verifier::verify(events)
}
