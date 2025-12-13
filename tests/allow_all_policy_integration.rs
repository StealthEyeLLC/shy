use whitehat::{AllowAllPolicy, Policy, Decision};

#[test]
fn allow_all_policy_integrates_cleanly() {
    let policy = AllowAllPolicy::new();

    let decision = policy.evaluate("any-input").unwrap();

    assert_eq!(decision, Decision::Allow);
}
