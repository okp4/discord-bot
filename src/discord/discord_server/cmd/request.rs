//! Holds request commands functions

use crate::cosmos::faucet::messages::request_funds::{RequestFunds, RequestFundsResult};
use crate::cosmos::tx::error::Error as RequestError;
use crate::cosmos::tx::error::Error::Mailbox;
use crate::discord::discord_server::cmd::CommandExecutable;
use crate::discord::discord_server::error::Error;
use crate::discord::discord_server::Actors;
use actix::MailboxError;
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

        let msg = if let Ok(address) = self.address.parse() {
            let request: Result<RequestFundsResult, MailboxError> = actors
                .faucet
                .send(RequestFunds {
                    address,
                    requester: command.user.clone(),
                })
                .await;
            match request.map_err(|e| Mailbox(e.to_string())).and_then(|r| r) {
                Ok(_) => "📥 Funds has been successfully requested.".to_string(),
                Err(err) => match err {
                    RequestError::QueueFull => "❌ Queue is full, please wait before re-submit funds request.".to_string(),
                    RequestError::DuplicateUser => "❌ You have already request funds, please wait before re-submit your request.".to_string(),
                    _ => "❌ An error occurs, please try again.".to_string(),
                }
            }
        } else {
            format!(
                "❌ Your wallet address `{}` seems to be wrong.",
                self.address
            )
        };

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.ephemeral(true).content(msg))
            })
            .await
            .map_err(Error::from)
    }
}
