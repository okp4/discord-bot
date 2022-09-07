//! Lite Discord Actor Client

mod actor;
mod handlers;
pub mod messages;

/// Discord actor client
pub struct DiscordActor {
    token: String,
}

impl DiscordActor {
    /// Create a new discord actor client
    pub fn new(token: String) -> Self {
        DiscordActor { token }
    }
}
