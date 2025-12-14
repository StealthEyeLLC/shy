//! DOC v0 — System Description SSPU
//!
//! Purpose (v0):
//! - Describe what the StealthEye system IS.
//! - Describe what components exist and their fixed roles.
//! - NO execution.
//! - NO authority.
//! - NO speculation.
//! - NO future planning.
//!
//! Output is a static, factual description of the system state.

/// Return the canonical system description for StealthEye v0.11.
pub fn system_description() -> String {
    let mut out = String::new();

    out.push_str("=== STEALTHEYE SYSTEM DESCRIPTION ===\n\n");

    out.push_str("System Identity:\n");
    out.push_str("  StealthEye is a governed execution platform.\n");
    out.push_str("  It is not autonomous.\n");
    out.push_str("  It is not an AI system.\n");
    out.push_str("  It executes only explicitly approved, deterministic operations.\n\n");

    out.push_str("Architecture Version:\n");
    out.push_str("  Architecture locked at v0.11.\n");
    out.push_str("  Core responsibilities are frozen.\n");
    out.push_str("  Changes are additive only.\n\n");

    out.push_str("SSPU Set (Authoritative Order):\n");
    out.push_str("  TME  — Time and ordering control\n");
    out.push_str("  OBS  — Observation (record-only facts)\n");
    out.push_str("  VFY  — Verification (prove claims)\n");
    out.push_str("  HBR  — Zero Trust approval gate\n");
    out.push_str("  SHY  — Deterministic execution kernel\n");
    out.push_str("  WDN  — Enforcement / containment\n");
    out.push_str("  CLR  — Cleanup / hygiene / recovery signaling\n");
    out.push_str("  CHK  — Invariant checking\n");
    out.push_str("  EXPL — Human-readable explanation\n");
    out.push_str("  DOC  — System description (this output)\n\n");

    out.push_str("Authority Rules:\n");
    out.push_str("  SHY is the sole executor.\n");
    out.push_str("  Domains are tools of SHY and hold no authority.\n");
    out.push_str("  SSPUs govern flow, not behavior.\n");
    out.push_str("  No silent behavior is permitted.\n\n");

    out.push_str("Integration Law:\n");
    out.push_str("  One-way flow only.\n");
    out.push_str("  No cycles.\n");
    out.push_str("  Integration order follows dependency gravity.\n\n");

    out.push_str("Current State:\n");
    out.push_str("  OBS v0 implemented.\n");
    out.push_str("  VFY v0 implemented.\n");
    out.push_str("  HBR v0 implemented.\n");
    out.push_str("  WDN v0 implemented.\n");
    out.push_str("  CLR v0 implemented.\n");
    out.push_str("  CHK v0 implemented.\n");
    out.push_str("  EXPL v0 implemented.\n");
    out.push_str("  DOC v0 implemented.\n\n");

    out.push_str("This description is factual and complete.\n");
    out.push_str("No future behavior is implied.\n");
    out.push_str("=== END SYSTEM DESCRIPTION ===\n");

    out
}
