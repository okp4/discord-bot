//! Holds some types for discord slash commands.

pub mod ping;
pub mod request;

use crate::discord_server::error::Error;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use strum_macros::Display;
use strum_macros::EnumString;

/// `CommandExecutable` trait is used to make a discord command execuable by the bot.
#[async_trait]
pub trait CommandExecutable {
    /// Execute command
    async fn execute(
        &self,
        _: &Context,
        _: &Interaction,
        _: &ApplicationCommandInteraction,
    ) -> Result<(), Error>;
}
/// The different supported commands.
#[derive(Display, EnumString)]
pub enum DiscordCommand {
    /// The ping command.
    #[strum(serialize = "ping")]
    Ping,
    /// The request command.
    #[strum(serialize = "request")]
    Request,
}
