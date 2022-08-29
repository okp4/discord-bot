//! Hold the ping command functions
use crate::grpc::client::Client as GRPCClient;
use crate::discord::cmd::CommandExecutable;
use crate::discord::error::Error;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use tonic::transport::Channel;

/// A command to send a ping request
pub struct PingCmd {}

/// Execute the "ping" command.
#[async_trait]
impl CommandExecutable for PingCmd {
    async fn execute(
        &self,
        ctx: &Context,
        _: &Interaction,
        command: &ApplicationCommandInteraction,
        _: &GRPCClient<Channel>,
    ) -> Result<(), Error> {
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("🏓 pong!".to_string()))
            })
            .await
            .map_err(Error::from)
    }
}
