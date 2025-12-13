use whitehat::RefusalReason;

#[test]
fn refusal_reason_is_constructible() {
    let _ = RefusalReason::PolicyDenied;
}
