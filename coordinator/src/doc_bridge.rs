//! Coordinator ↔ DOC Bridge (v0)
//!
//! Purpose:
//! - Expose the canonical system description.
//! - NO execution authority.
//! - Static, deterministic output only.

use doc::system_description;

pub fn describe_system() -> String {
    system_description()
}
