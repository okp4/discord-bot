//! Lite Discord Actor Client

use crate::discord::discord_client::message::DiscordMessage;
use std::marker::PhantomData;

mod actor;
mod handlers;
pub mod message;
pub mod messages;

/// Discord actor client
pub struct DiscordActor<M>
where
    M: DiscordMessage,
{
    token: String,
    phantom: PhantomData<M>,
}

impl<M> DiscordActor<M>
where
    M: DiscordMessage,
{
    /// Create a new discord actor client
    pub fn new(token: String) -> Self {
        DiscordActor {
            token,
            phantom: PhantomData,
        }
    }
}
