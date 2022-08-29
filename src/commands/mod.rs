//! DiscordBot Subcommands

mod start;

use self::start::StartCmd;
use crate::config::DiscordBotConfig;
use abscissa_core::{config::Override, Command, Configurable, FrameworkError, Runnable};
use clap::Parser;
use std::path::PathBuf;

/// DiscordBot Configuration Filename
pub const CONFIG_FILE: &str = "discord_bot.toml";

/// DiscordBot Subcommands
#[derive(Command, Debug, Parser, Runnable)]
pub enum DiscordBotCmd {
    /// Boot the discord bot
    Start(StartCmd),
}

/// OKP4 discord bot application.
#[derive(Command, Debug, Parser)]
#[clap(author, about, version)]
pub struct EntryPoint {
    #[clap(subcommand)]
    cmd: DiscordBotCmd,

    /// Enable verbose logging
    #[clap(short, long)]
    pub verbose: bool,

    /// Use the specified config file
    #[clap(short, long)]
    pub config: Option<String>,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        self.cmd.run()
    }
}

/// Define how application configuration is loaded.
impl Configurable<DiscordBotConfig> for EntryPoint {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        let filename = self
            .config
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| CONFIG_FILE.into());

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    fn process_config(&self, config: DiscordBotConfig) -> Result<DiscordBotConfig, FrameworkError> {
        match &self.cmd {
            DiscordBotCmd::Start(cmd) => cmd.override_config(config),
        }
    }
}