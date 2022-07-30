//! DiscordBot Config
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
/// Configuration type for the discord bot
pub struct DiscordBotConfig {
    /// discord section
    pub discord: DiscordSection,

    /// Metrics configuration
    pub metrics: MetricsSection,
}

/// Discord section.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiscordSection {
    /// Token
    pub token: String,

    /// Guild ID (Server ID)
    pub guild_id: u64,
}

impl Default for DiscordSection {
    fn default() -> Self {
        Self {
            token: "".to_owned(),
            guild_id: 0,
        }
    }
}

/// Metrics configuration section.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MetricsSection {
    /// The address used for the Prometheus metrics endpoint.
    pub endpoint: Option<SocketAddr>,
}
