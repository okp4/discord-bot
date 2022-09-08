use crate::discord::discord_client::message::DiscordMessage;
use actix::{Actor, Context};

use super::DiscordActor;

impl<M> Actor for DiscordActor<M>
where
    M: DiscordMessage + Unpin + 'static,
{
    type Context = Context<Self>;
}
