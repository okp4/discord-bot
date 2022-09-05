//! Holds discord send message related types

use actix::Message;

/// Result of a discord message actor message
pub type SendMessageResult = ();

/// Send discord message actor message
#[derive(Message)]
#[rtype(result = "SendMessageResult")]
pub struct SendMessage {
    /// Message to send
    pub content: String,
    /// Channel to send into
    pub channel_id: u64,
}
