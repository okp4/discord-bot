//! Discord bot implementations
use crate::discord::cmd_ping;
use crate::discord::error::Error as DiscordError;
use crate::discord::error::ErrorKind::UnknownCommand;
use crate::discord::metrics::{
    LABEL_NAME_COMMAND, LABEL_NAME_INTERACTION, LABEL_VALUE_COMMAND_UNKNOWN,
    METRIC_DISCORD_INTERACTIONS_DURATION, METRIC_DISCORD_INTERACTIONS_TOTAL,
};
use crate::discord::utils::interation_name;
use crate::error::{Error, ErrorKind};
use metrics::{describe_counter, describe_histogram, histogram, increment_counter, Unit};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::{async_trait, model::id::GuildId};
use std::process::exit;
use std::str::FromStr;
use std::time::Instant;

use crate::discord::cmd::DiscordCommand;
use tracing::{debug, error, info, warn};

struct Handler {
    guild_id: GuildId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("🤝 {} is connected!", ready.user.name);

        let commands = GuildId::set_application_commands(&self.guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name(DiscordCommand::Ping)
                    .description("A ping command 🏓 (for testing purposes)")
            })
        })
        .await;

        match commands {
            Ok(commands) => {
                info!(
                    "💻 I now have the following guild slash commands: {:?}",
                    commands
                );
            }
            Err(err) => {
                error!("💀 Failed to create application commands: {}", err);
                exit(-1);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let start = Instant::now();

        let labels = match interaction {
            Interaction::Ping(_)
            | Interaction::MessageComponent(_)
            | Interaction::Autocomplete(_)
            | Interaction::ModalSubmit(_) => {
                vec![(LABEL_NAME_INTERACTION, interation_name(&interaction))]
            }
            Interaction::ApplicationCommand(ref command) => {
                info!(
                    "➡️ Received command interaction: {} ({}) from {}",
                    command.data.name, command.id, command.user.name
                );
                debug!("🔬Command is: {:#?}", command);

                let discord_command = DiscordCommand::from_str(&command.data.name);
                let labels = vec![
                    (LABEL_NAME_INTERACTION, interation_name(&interaction)),
                    (
                        LABEL_NAME_COMMAND,
                        discord_command
                            .as_ref()
                            .map(|name| name.to_string())
                            .unwrap_or_else(|_| LABEL_VALUE_COMMAND_UNKNOWN.to_string()),
                    ),
                ];

                let execution_result = match discord_command {
                    Ok(DiscordCommand::Ping) => cmd_ping::execute(&ctx, &interaction, command),
                    _ => Err(DiscordError::from(UnknownCommand(format!(
                        "🤔 I don't understand: {}",
                        command.data.name
                    )))),
                };

                match execution_result {
                    Ok(content) => {
                        if let Err(why) = Self::send_response(&ctx, command, content).await {
                            warn!("❌ Cannot respond to slash command: {}", why);
                        }
                    }
                    Err(err) => {
                        warn!("❌ Failed to execute command: {}", err);

                        let content =
                            format!("😖I failed to execute the command! (error was: {})", err);
                        if let Err(why) = Self::send_response(&ctx, command, content).await {
                            warn!("❌ Cannot respond to slash command: {}", why);
                        }
                    }
                }

                labels.to_vec()
            }
        };
        let delta = start.elapsed();

        increment_counter!(METRIC_DISCORD_INTERACTIONS_TOTAL, &labels);
        histogram!(METRIC_DISCORD_INTERACTIONS_DURATION, delta, &labels);
    }
}

impl Handler {
    async fn send_response(
        ctx: &Context,
        command: &ApplicationCommandInteraction,
        content: String,
    ) -> serenity::Result<()> {
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await
    }
}

fn register_metrics() {
    describe_counter!(
        METRIC_DISCORD_INTERACTIONS_TOTAL,
        "The total number of interactions received by the bot from Discord, labeled with: interaction, command."
    );
    describe_histogram!(
        METRIC_DISCORD_INTERACTIONS_DURATION,
        Unit::Seconds,
        "Timing statistics (percentiles) for Discord interaction processing durations, labeled with: interaction, command, quantile."
    );
}

/// Start the discord bot (given a token)
pub async fn start(token: &str, guild_id: u64) -> Result<(), Error> {
    register_metrics();

    let intents = GatewayIntents::empty();

    info!("🚀 Booting the Bot...");

    let result = Client::builder(&token, intents)
        .event_handler(Handler {
            guild_id: GuildId(guild_id),
        })
        .await
        .map_err(|_| Error::from(ErrorKind::Client("Failed to create client".to_owned())));

    match result {
        Ok(mut client) => client.start().await.map_err(Error::from),
        r => r.map(|_| ()),
    }
}
