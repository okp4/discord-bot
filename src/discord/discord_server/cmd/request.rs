//! Holds request commands functions

use crate::cosmos::client::error::Error as ChainError;
use crate::cosmos::faucet::messages::request_funds::RequestFunds;
use crate::discord::discord_server::cmd::CommandExecutable;
use crate::discord::discord_server::error::{Error, ErrorKind};
use crate::discord::discord_server::Actors;
use cosmrs::Error as CosmosError;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use tracing::info;

/// A command to ask chain to receive token
pub struct RequestCmd {
    /// Wallet address which will receive token
    pub(crate) address: String,
}

/// Execute the "ping" command.
#[async_trait]
impl CommandExecutable for RequestCmd {
    async fn execute(
        &self,
        ctx: &Context,
        _: &Interaction,
        command: &ApplicationCommandInteraction,
        actors: &Actors,
    ) -> Result<(), Error> {
        info!("💰 request fund slash command");
        actors.faucet.do_send(RequestFunds {
            address: self.address.parse().map_err(|_| {
                Error::from(ErrorKind::Chain(ChainError::Cosmos(
                    CosmosError::AccountId {
                        id: self.address.to_string(),
                    },
                )))
            })?,
            requester: command.user.clone(),
        });

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .ephemeral(true)
                            .content("📥 Funds has been successfully requested.".to_string())
                    })
            })
            .await
            .map_err(Error::from)
    }
}