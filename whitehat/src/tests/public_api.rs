use whitehat::{Decision, Policy};

struct TestPolicy;

impl Policy for TestPolicy {
    fn evaluate(&self, _input: &str) -> Result<Decision, whitehat::WhiteHatError> {
        Ok(Decision::Allow)
    }
}

#[test]
fn policy_can_be_called() {
    let p = TestPolicy;
    let _ = p.evaluate("test");
}
