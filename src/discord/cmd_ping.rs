//! The "ping" slash command implementation.
use crate::discord::cmd::CommandExecutionResult;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;

/// Execute the "ping" command.
pub fn execute(
    _: &Context,
    _: &Interaction,
    _: &ApplicationCommandInteraction,
) -> CommandExecutionResult {
    Ok("🏓 pong!".to_string())
}
