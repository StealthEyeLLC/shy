use crate::CoordinatorError;
use whitehat::Policy;

/// Public coordinator entry surface
pub struct Coordinator {
    policy: Box<dyn Policy>,
}

impl Coordinator {
    pub fn new(policy: Box<dyn Policy>) -> Self {
        Self { policy }
    }

    pub fn policy(&self) -> &dyn Policy {
        &*self.policy
    }

    pub fn validate(&self) -> Result<(), CoordinatorError> {
        Ok(())
    }
}
