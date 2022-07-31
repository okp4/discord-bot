//! Discord bot implementations
use crate::error::{Error, ErrorKind};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::{async_trait, model::id::GuildId};
use std::process::exit;
use std::str::FromStr;
use strum_macros::Display;
use strum_macros::EnumString;
use tracing::{debug, error, info, warn};

struct Handler {
    guild_id: GuildId,
}

#[derive(Display, EnumString)]
enum DiscordCommand {
    #[strum(serialize = "ping")]
    Ping,
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
        if let Interaction::ApplicationCommand(command) = interaction {
            info!(
                "➡️ Received command interaction: {} ({}) from {}",
                command.data.name, command.id, command.user.name
            );

            debug!("🔬Command is: {:#?}", command);

            let content = match DiscordCommand::from_str(&command.data.name) {
                Ok(DiscordCommand::Ping) => "🏓 pong!".to_string(),
                _ => format!("🤔 I don't understand: {}", command.data.name),
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
        }
    }
}

/// Start the discord bot (given a token)
pub async fn start(token: &str, guild_id: u64) -> Result<(), Error> {
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
