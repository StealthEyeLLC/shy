//! Smoke test for SSPU chain wiring.
//!
//! This test does NOT verify behavior.
//! It only ensures the system links and executes without panic.

use chk::Checker;
use clr::Cleaner;
use expl::explain;
use hbr::Gate;
use obs::{ObsEvent, ObsLog};
use vfy::Verifier;
use wdn::Enforcer;

#[test]
fn sspu_chain_smoke_test() {
    // OBS
    let obs = ObsLog::new();
    obs.record("TEST", "smoke test event");

    let events: Vec<ObsEvent> = obs.snapshot();

    // VFY
    let vfy_result = Verifier::verify(&events);

    // HBR
    let hbr_outcome = Gate::decide(&vfy_result);

    // WDN
    let enforcement = Enforcer::enforce(&hbr_outcome);

    // CLR
    let cleanup = Cleaner::resolve(&enforcement);

    // CHK
    let invariant_report = Checker::check(&cleanup);

    // EXPL
    let explanation = explain(
        &events,
        &vfy_result,
        &hbr_outcome,
        &enforcement,
        &cleanup,
        &invariant_report,
    );

    // Minimal assertion: explanation must exist
    assert!(!explanation.is_empty());
}
