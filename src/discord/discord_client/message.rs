//! Contains definition of a discord message

use serenity::async_trait;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::Error;

/// Trait representing a disorcd message
#[async_trait]
pub trait DiscordMessage {
    /// Return the channel id relative to send discord message.
    fn channel_id(&self) -> u64;
    /// Send the message to discord
    async fn send_message(self, http: &Http) -> Result<Message, Error>;
}
