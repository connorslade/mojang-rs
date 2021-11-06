use std::time::Duration;

use ureq::Agent;
use ureq::AgentBuilder;

/// Defult Agent for all Requests
pub fn ureq_agent() -> Agent {
    AgentBuilder::new()
        .timeout(Duration::from_secs(4))
        .user_agent("rust-mojang/0.0.0")
        .build()
}
