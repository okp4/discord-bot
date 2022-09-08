//! Holds discord send message related types

use crate::discord::discord_client::message::DiscordMessage;
use actix::Message;

/// Result of a discord message actor message
pub type SendMessageResult = ();

/// Send discord message actor message
#[derive(Message)]
#[rtype(result = "SendMessageResult")]
pub struct SendMessage<M>
where
    M: DiscordMessage,
{
    /// Message to send to discord
    pub message: M,
}
