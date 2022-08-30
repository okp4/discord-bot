//! Discord bot implementations
use crate::cosmos::client::error::Error as ChainError;
use crate::cosmos::client::Client as GRPCClient;
use crate::discord::cmd::ping::PingCmd;
use crate::discord::cmd::request::RequestCmd;
use crate::discord::cmd::CommandExecutable;
use crate::discord::cmd::DiscordCommand;
use crate::discord::error::Error as DiscordError;
use crate::discord::error::ErrorKind::IncorrectArg;
use crate::discord::error::ErrorKind::MissingArg;
use crate::discord::error::ErrorKind::UnknownCommand;
use crate::discord::metrics::{
    LABEL_NAME_COMMAND, LABEL_NAME_INTERACTION, LABEL_VALUE_COMMAND_UNKNOWN,
    METRIC_DISCORD_INTERACTIONS_DURATION, METRIC_DISCORD_INTERACTIONS_TOTAL,
};
use crate::discord::utils::interation_name;
use crate::error::{Error, ErrorKind};
use metrics::{describe_counter, describe_histogram, histogram, increment_counter, Unit};
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::{async_trait, model::id::GuildId};
use std::process::exit;
use std::str::FromStr;
use std::time::Instant;
use tonic::transport::Channel;

use crate::cli::prelude::*;
use tracing::{debug, error, info, warn};

struct Handler {
    guild_id: GuildId,
    grpc_client: GRPCClient<Channel>,
}

impl Handler {
    async fn new(guild_id: GuildId, grpc_address: String) -> Result<Handler, ChainError> {
        let grpc_client = GRPCClient::new(grpc_address).await?;
        Ok(Handler {
            guild_id,
            grpc_client,
        })
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("🤝 {} is connected!", ready.user.name);

        let commands = GuildId::set_application_commands(&self.guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command
                        .name(DiscordCommand::Ping)
                        .description("A ping command 🏓 (for testing purposes)")
                })
                .create_application_command(|command| {
                    command
                        .name(DiscordCommand::Request)
                        .description("Request 1know from testnet 💵")
                        .create_option(|option| {
                            option
                                .name("address")
                                .description("OKP4 address you want to receive know")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
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

                let execution_result: Result<(), DiscordError> = match discord_command {
                    Ok(DiscordCommand::Ping) => {
                        PingCmd {}
                            .execute(&ctx, &interaction, command, &self.grpc_client)
                            .await
                    }
                    Ok(DiscordCommand::Request) => {
                        match command
                            .data
                            .options
                            .first()
                            .and_then(|v| v.value.as_ref())
                            .ok_or_else(|| DiscordError::from(MissingArg("address".to_string())))
                            .and_then(|v| {
                                v.as_str().ok_or_else(|| {
                                    DiscordError::from(IncorrectArg(
                                        "address".to_string(),
                                        "Should be a string".to_string(),
                                    ))
                                })
                            })
                            .map(|v| v.to_string())
                            .map(|address| {
                                info!("Request command to address : {}", address);
                                RequestCmd { address }
                            }) {
                            Ok(cmd) => {
                                cmd.execute(&ctx, &interaction, command, &self.grpc_client)
                                    .await
                            }
                            Err(why) => Err(why),
                        }
                    }
                    _ => Err(DiscordError::from(UnknownCommand(format!(
                        "🤔 I don't understand: {}",
                        command.data.name
                    )))),
                };

                match execution_result {
                    Ok(_) => {
                        info!("✅ Successful execute slash command");
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
pub async fn start(token: &str, guild_id: u64, shard: u64, shards: u64) -> Result<(), Error> {
    register_metrics();

    let intents = GatewayIntents::empty();

    info!("🪐 Start connection to cosmos grpc endpoint");

    let result = match Handler::new(
        GuildId(guild_id),
        APP.config().chain.grpc_address.to_string(),
    )
    .await
    {
        Ok(handler) => {
            info!("🛰 Connection to cosmos grpc endpoint successful");
            info!("🚀 Booting the Bot...");

            Client::builder(&token, intents)
                .event_handler(handler)
                .await
                .map_err(|_| Error::from(ErrorKind::Client("Failed to create client".to_string())))
        }
        Err(why) => {
            error!("❌ Failed connection to grpc endpoint: {}", why);
            Err(Error::from(ErrorKind::Client(
                "Failed launch bot without grpc connection".to_string(),
            )))
        }
    };

    match result {
        Ok(mut client) => client.start_shard(shard, shards).await.map_err(Error::from),
        r => r.map(|_| ()),
    }
}
