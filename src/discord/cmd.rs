//! Holds some types for discord slash commands.

pub(crate) mod ping;
pub(crate) mod request;

use crate::chain::client::Client as GRPCClient;
use crate::discord::cmd::ping::PingCmd;
use crate::discord::cmd::request::RequestCmd;
use crate::discord::error::Error;
use crate::discord::error::ErrorKind::{IncorrectArg, MissingArg, UnknownCommand};
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use strum_macros::Display;
use tonic::transport::Channel;

/// `CommandExecutable` trait is used to make a discord command execuable by the bot.
#[async_trait]
pub trait CommandExecutable {
    /// Execute command
    async fn execute(
        &self,
        _: &Context,
        _: &Interaction,
        _: &ApplicationCommandInteraction,
        _: &GRPCClient<Channel>,
    ) -> Result<(), Error>;
}
/// The different supported commands.
#[derive(Display)]
pub enum DiscordCommand {
    /// The ping command.
    #[strum(serialize = "ping")]
    Ping(PingCmd),
    /// The request command.
    #[strum(serialize = "request")]
    Request(RequestCmd),
}

impl DiscordCommand {
    /// Create new discord command based on command name and argument
    pub fn new(cmd: &str, interaction: &ApplicationCommandInteraction) -> Result<Self, Error> {
        match cmd {
            "ping" => Ok(DiscordCommand::Ping(PingCmd {})),
            "request" => interaction
                .data
                .options
                .first()
                .and_then(|v| v.value.as_ref())
                .ok_or_else(|| Error::from(MissingArg("address".to_string())))
                .and_then(|v| {
                    v.as_str().ok_or_else(|| {
                        Error::from(IncorrectArg(
                            "address".to_string(),
                            "Should be a string".to_string(),
                        ))
                    })
                })
                .map(|v| v.to_string())
                .map(|address| DiscordCommand::Request(RequestCmd { address })),
            _ => Err(Error::from(UnknownCommand(format!(
                "🤔 I don't understand: {}",
                interaction.data.name
            )))),
        }
    }
}

#[async_trait]
impl CommandExecutable for DiscordCommand {
    async fn execute(
        &self,
        ctx: &Context,
        interaction: &Interaction,
        command: &ApplicationCommandInteraction,
        grpc_client: &GRPCClient<Channel>,
    ) -> Result<(), Error> {
        match self {
            DiscordCommand::Ping(cmd) => cmd.execute(ctx, interaction, command, grpc_client).await,
            DiscordCommand::Request(cmd) => {
                cmd.execute(ctx, interaction, command, grpc_client).await
            }
        }
    }
}
