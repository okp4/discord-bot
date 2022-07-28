//! DiscordBot Config
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]

/// Configuration type for the discord bot
pub struct DiscordBotConfig {
    /// discord section
    pub discord: DiscordSection,
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
