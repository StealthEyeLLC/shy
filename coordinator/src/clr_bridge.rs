//! Coordinator ↔ CLR Bridge (v0)
//!
//! Purpose:
//! - Run cleanup resolution after enforcement.
//! - NO execution authority.
//! - Explicit, deterministic call only.

use clr::{Cleaner, CleanupResult};
use wdn::EnforcementResult;

pub fn resolve(enforcement: &EnforcementResult) -> CleanupResult {
    Cleaner::resolve(enforcement)
}
