//! Discord bot implementations
use crate::error::{Error, ErrorKind};
use metrics::{describe_counter, describe_histogram, histogram, increment_counter, Unit};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::{async_trait, model::id::GuildId};
use std::process::exit;
use std::str::FromStr;
use std::time::Instant;
use strum_macros::Display;
use strum_macros::EnumString;
use tracing::{debug, error, info, warn};

static METRIC_DISCORD_INTERACTIONS_TOTAL: &str = "discord_interactions_total";
static METRIC_DISCORD_INTERACTIONS_DURATION: &str = "discord_interactions_duration";

struct Handler {
    guild_id: GuildId,
}

#[derive(Display, EnumString)]
enum DiscordCommand {
    #[strum(serialize = "ping")]
    Ping,
}

#[inline(always)]
fn make_label<'a>(key: &'a str, value: &str) -> (&'a str, String) {
    (key, value.to_string())
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
            Interaction::Ping(_) => [make_label("interaction", "ping")].to_vec(),
            Interaction::MessageComponent(_) => {
                [make_label("interaction", "message-component")].to_vec()
            }
            Interaction::Autocomplete(_) => [make_label("interaction", "autocomplete")].to_vec(),
            Interaction::ModalSubmit(_) => [make_label("interaction", "modal-submit")].to_vec(),
            Interaction::ApplicationCommand(command) => {
                info!(
                    "➡️ Received command interaction: {} ({}) from {}",
                    command.data.name, command.id, command.user.name
                );
                debug!("🔬Command is: {:#?}", command);

                let (content, labels) = match DiscordCommand::from_str(&command.data.name) {
                    Ok(DiscordCommand::Ping) => {
                        let labels = [
                            make_label("interaction", "application-command"),
                            ("command", command.data.name.clone()),
                        ];

                        ("🏓 pong!".to_string(), labels)
                    }
                    _ => {
                        let labels = [
                            make_label("interaction", "application-command"),
                            make_label("command", "???"),
                        ];

                        (
                            format!("🤔 I don't understand: {}", command.data.name),
                            labels,
                        )
                    }
                };

                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(content))
                    })
                    .await
                {
                    warn!("❌ Cannot respond to slash command: {}", why);
                }

                labels.to_vec()
            }
        };
        let delta = start.elapsed();

        increment_counter!(METRIC_DISCORD_INTERACTIONS_TOTAL, &labels);
        histogram!(METRIC_DISCORD_INTERACTIONS_DURATION, delta, &labels);
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
