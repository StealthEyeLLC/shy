use whitehat::StackedPolicy;

#[test]
fn stacked_policy_constructs() {
    let policies = Vec::new();
    let sp = StackedPolicy::new(policies);
    assert_eq!(sp.policies_len(), 0);
}
