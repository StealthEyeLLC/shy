// PUBLIC API FROZEN — changes require version bump
use coordinator::FlowState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExitCode {
    Success,
    Failure,
}

pub struct Cli;

impl Cli {
    pub fn run(&self, _args: &[String]) -> Result<ExitCode, FlowState> {
        Ok(ExitCode::Success)
    }
}
