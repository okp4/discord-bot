use actix::{Actor, Context};

use super::DiscordActor;

impl Actor for DiscordActor {
    type Context = Context<Self>;
}
