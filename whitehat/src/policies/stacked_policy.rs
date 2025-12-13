use crate::{Decision, Policy, WhiteHatError};

/// PUBLIC API FROZEN — changes require version bump
pub struct StackedPolicy {
    policies: Vec<Box<dyn Policy>>,
}

impl StackedPolicy {
    pub fn new(policies: Vec<Box<dyn Policy>>) -> Self {
        StackedPolicy { policies }
    }

    pub fn policies_len(&self) -> usize {
        self.policies.len()
    }
}

impl Policy for StackedPolicy {
    fn evaluate(&self, input: &str) -> Result<Decision, WhiteHatError> {
        for policy in &self.policies {
            match policy.evaluate(input)? {
                Decision::Allow => continue,
                Decision::Deny => return Ok(Decision::Deny),
            }
        }

        Ok(Decision::Allow)
    }
}
