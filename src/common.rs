#[cfg(feature = "timeout")]
use std::time::Duration;

use ureq::Agent;
use ureq::AgentBuilder;

const USER_AGENT: &str = concat!("mojang-rs/", env!("CARGO_PKG_VERSION"));

/// Default Agent for all Requests
pub fn ureq_agent() -> Agent {
    #[cfg(feature = "timeout")]
    return AgentBuilder::new()
        .timeout(Duration::from_secs(4))
        .user_agent(USER_AGENT)
        .build();

    #[cfg(not(feature = "timeout"))]
    return AgentBuilder::new().user_agent(USER_AGENT).build();
}
