use crate::CoordinatorError;
use whitehat::Policy;

/// End-to-end orchestration flow
pub struct EndToEndFlow {
    policy: Box<dyn Policy>,
}

impl EndToEndFlow {
    pub fn new(policy: Box<dyn Policy>) -> Self {
        Self { policy }
    }

    pub fn run(&self) -> Result<(), CoordinatorError> {
        Ok(())
    }
}
