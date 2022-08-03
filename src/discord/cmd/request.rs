use abscissa_core::Application;
use crate::discord::cmd::CommandExecutable;
use crate::discord::error::Error;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use crate::application::APP;
use crate::chain::faucet::FaucetClient;

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
    ) -> Result<(), Error> {

        let config = &APP.config();

        let _ =FaucetClient::new(&config.faucet.mnemonic)?;

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content(format!("Request token for {}", self.address))
                    })
            })
            .await
            .map_err(Error::from)
    }
}
