//! DiscordBot Config
use serde::{Deserialize, Serialize};

/// DiscordBot Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiscordBotConfig {
    /// An example configuration section
    pub hello: ExampleSection,
}

/// Default configuration settings.
impl Default for DiscordBotConfig {
    fn default() -> Self {
        Self {
            hello: ExampleSection::default(),
        }
    }
}

/// Example configuration section.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExampleSection {
    /// Example configuration value
    pub recipient: String,
}

impl Default for ExampleSection {
    fn default() -> Self {
        Self {
            recipient: "world".to_owned(),
        }
    }
}
