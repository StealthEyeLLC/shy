//! Coordinator ↔ CHK Bridge (v0)
//!
//! Purpose:
//! - Run invariant checks after cleanup.
//! - NO execution authority.
//! - Explicit, deterministic call only.

use chk::{Checker, InvariantReport};
use clr::CleanupResult;

pub fn check(result: &CleanupResult) -> InvariantReport {
    Checker::check(result)
}
