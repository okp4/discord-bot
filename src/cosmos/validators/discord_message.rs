//! Implementation of a validators state discord message

use serenity::async_trait;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::Error;

use crate::discord::discord_client::message::DiscordMessage;

static MESSAGE_TITLE: &str = "Validator state changed";

/// Validator changing state message
#[derive(Clone, Debug)]
pub struct ValidatorsMessage {
    /// Description of the embedded message - optional
    pub description: String,
    /// Channel to send into
    pub channel_id: u64,
}

#[async_trait]
impl DiscordMessage for ValidatorsMessage {
    async fn send_message(self, http: &Http) -> Result<Message, Error> {
        ChannelId(self.channel_id)
            .send_message(&http, |m| {
                m.embed(|e| {
                    e.title(MESSAGE_TITLE.to_string());
                    e.description(self.description);
                    e
                });
                m
            })
            .await
    }
}
