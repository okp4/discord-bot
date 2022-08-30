//! `start` subcommand
use std::net::SocketAddr;
use std::process;

use abscissa_core::{config, Command, FrameworkError, FrameworkErrorKind, Runnable};
use clap::Parser;
use tracing::{error, info};

use crate::{
    cli::{
        config::{DiscordBotConfig, DiscordShardingSection},
        prelude::*,
    },
    discord::client,
};

#[derive(Command, Debug, Parser)]
#[clap(arg_required_else_help(true))]
pub struct StartCmd {
    /// The discord token
    #[clap(short = 't', long = "token")]
    token: Option<String>,

    /// The guild ID (Server ID)
    #[clap(short = 'g', long = "guild-id")]
    guild_id: Option<u64>,

    /// The shard index ID to start.
    /// Establish a sharded connection and start listening for events.
    /// This will start receiving events and dispatch them to your registered handlers.
    /// This will create a single shard by ID. If using one shard per process, you will need to start other bot process with the other shard IDs.
    #[clap(long)]
    shard: Option<u64>,

    /// The total numbers of shards in the sharding connection.
    #[clap(long)]
    shards: Option<u64>,

    /// The prometheus endpoint.
    /// Optional. Configures an HTTP exporter that functions as a scrape endpoint for prometheus.
    /// The value is an IPv4 or IPv6 address and a port number, separated by a colon. For instance:
    /// 0.0.0.0:9000
    #[clap(short = 'p', long = "prometheus-endpoint")]
    prometheus_endpoint: Option<SocketAddr>,
}

impl Runnable for StartCmd {
    fn run(&self) {
        let config = APP.config();

        abscissa_tokio::run(&APP, async {
            match client::start(
                &config.discord.token,
                config.discord.guild_id,
                config.discord.sharding.shard,
                config.discord.sharding.shards,
            )
            .await
            {
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

        match (self.shard, self.shards) {
            (Some(shard), Some(shards)) => {
                config.discord.sharding = DiscordShardingSection { shard, shards }
            }
            (None, Some(shards)) => {
                Err(FrameworkError::from(FrameworkErrorKind::ConfigError.context(format!("❌ When set the `shards` ({}) attribute, you should also set the `shard` index", shards))))?;
            }
            (Some(shard), None) => {
                Err(FrameworkError::from(FrameworkErrorKind::ConfigError.context(format!("❌ When set the `shard` ({}) index, you should also set the total number of `shards` (`--shards <NUMBER_OF_SHARD>`)", shard))))?;
            }
            _ => (),
        };

        if self.prometheus_endpoint.is_some() {
            config.metrics.endpoint = self.prometheus_endpoint
        }

        Ok(config)
    }
}
