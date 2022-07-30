//! `start` subcommand
use std::net::SocketAddr;
use std::process;

use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::Parser;
use tracing::{error, info};

use crate::config::DiscordBotConfig;
use crate::discord;
use crate::prelude::*;

#[derive(Command, Debug, Parser)]
#[clap(arg_required_else_help(true))]
pub struct StartCmd {
    /// The discord token
    #[clap(short = 't')]
    token: Option<String>,

    /// The guild ID (Server ID)
    #[clap(short = 'g')]
    guild_id: Option<u64>,

    /// The prometheus endpoint.
    /// Optional. Configures an HTTP exporter that functions as a scrape endpoint for prometheus.
    /// The value is an IPv4 or IPv6 address and a port number, separated by a colon. For instance:
    /// 0.0.0.0:9000
    #[clap(short = 'p')]
    prometheus_endpoint: Option<SocketAddr>,
}

impl Runnable for StartCmd {
    fn run(&self) {
        let config = APP.config();

        abscissa_tokio::run(&APP, async {
            match discord::start(&config.discord.token, config.discord.guild_id).await {
                Err(why) => error!("💀 Client error: {:?}", why),
                _ => info!("👋 Bye!"),
            }
        })
        .unwrap_or_else(|e| {
            error!("💀 executor exited with error: {}", e);
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

        if self.prometheus_endpoint.is_some() {
            config.metrics.endpoint = self.prometheus_endpoint
        }

        Ok(config)
    }
}
