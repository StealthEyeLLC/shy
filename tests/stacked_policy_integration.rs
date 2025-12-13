use whitehat::{AllowAllPolicy, StackedPolicy, Policy, Decision};

#[test]
fn stacked_policy_denies_on_first_deny() {
    struct DenyAll;

    impl Policy for DenyAll {
        fn evaluate(&self, _input: &str) -> Result<Decision, whitehat::WhiteHatError> {
            Ok(Decision::Deny)
        }
    }

    let policies: Vec<Box<dyn Policy>> = vec![
        Box::new(AllowAllPolicy::new()),
        Box::new(DenyAll),
        Box::new(AllowAllPolicy::new()),
    ];

    let stacked = StackedPolicy::new(policies);

    let decision = stacked.evaluate("input").unwrap();
    assert_eq!(decision, Decision::Deny);
}

#[test]
fn stacked_policy_allows_if_all_allow() {
    let policies: Vec<Box<dyn Policy>> = vec![
        Box::new(AllowAllPolicy::new()),
        Box::new(AllowAllPolicy::new()),
    ];

    let stacked = StackedPolicy::new(policies);

    let decision = stacked.evaluate("input").unwrap();
    assert_eq!(decision, Decision::Allow);
}
