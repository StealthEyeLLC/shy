use cli::CliReport;
use coordinator::FlowState;
use whitehat::RefusalReason;

#[test]
fn cli_reports_policy_denial() {
    let report = CliReport::new();
    let result = report.map_flow_state(FlowState::Rejected);
    assert_eq!(result, Err(RefusalReason::PolicyDenied));
}

#[test]
#[should_panic(expected = "uninitialized")]
fn cli_panics_on_uninitialized_flow() {
    let report = CliReport::new();
    let _ = report.map_flow_state(FlowState::Initialized);
}
