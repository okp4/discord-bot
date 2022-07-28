//! `start` subcommand
use crate::prelude::*;

use crate::config::DiscordBotConfig;
use crate::discord;

use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::{Parser};
use std::process;
use tracing::{error, info};
#[derive(Command, Debug, Parser)]
#[clap(arg_required_else_help(true))]
pub struct StartCmd {
    /// The discord token
    #[clap(short = 't')]
    token: Option<String>,

    /// The guild ID (Server ID)
    #[clap(short = 'g')]
    guild_id: Option<u64>,
}

impl Runnable for StartCmd {
    fn run(&self) {
        let config = APP.config();

        abscissa_tokio::run(&APP, async {
            match discord::start(&config.discord.token).await {
                Err(why) => error!("💥 Client error: {:?}", why),
                _ => info!("👋 Bye!"),
            }
        }).unwrap_or_else(|e| {
            error!("💥 executor exited with error: {}", e);
            process::exit(1);
        });
    }
}

impl config::Override<DiscordBotConfig> for StartCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: DiscordBotConfig,
    ) -> Result<DiscordBotConfig, FrameworkError> {
        if let Some(token) = self.token.clone() {
            config.discord.token = token;
        }

        if let Some(guild_id) = self.guild_id {
            config.discord.guild_id = guild_id
        }

        Ok(config)
    }
}
