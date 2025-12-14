use obs::{ObsEvent, ObsLog};

/// Policy is the bounded rule/governance interface for the coordinator flow.
/// OBS does not influence Policy, ever.
pub trait Policy: Send + Sync {
    /// Execute the SHY-driven flow (or whatever your coordinator currently delegates to).
    /// Keep this deterministic and supervised.
    fn execute(&self);
}

/// End-to-end flow wrapper with record-only observability via OBS v0.
pub struct EndToEndFlow {
    policy: Box<dyn Policy>,
    obs: ObsLog,
}

impl EndToEndFlow {
    pub fn new(policy: Box<dyn Policy>) -> Self {
        Self {
            policy,
            obs: ObsLog::new(),
        }
    }

    /// Record-only execution trace around the delegated execution.
    /// OBS has no authority and does not affect execution.
    pub fn run(&self) {
        self.obs.record("COORD", "EndToEndFlow.run: begin");
        self.obs.record("COORD", "Delegated execute: begin");

        self.policy.execute();

        self.obs.record("COORD", "Delegated execute: end");
        self.obs.record("COORD", "EndToEndFlow.run: end");
    }

    /// Read-only access for verification/tests.
    pub fn obs_snapshot(&self) -> Vec<ObsEvent> {
        self.obs.snapshot()
    }
}
