use coordinator::FlowState;

use crate::{Cli, ExitCode};

/// PUBLIC API FROZEN — changes require version bump
pub struct CliFlow;

impl CliFlow {
    pub fn new() -> Self {
        CliFlow
    }

    pub fn execute(&self, cli: &Cli, args: &[String]) -> Result<ExitCode, FlowState> {
        let _ = (cli, args);
        todo!()
    }
}
