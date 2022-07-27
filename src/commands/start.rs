/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::DiscordBotConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::Parser;

/// `start` subcommand
#[derive(Command, Debug, Parser)]
pub struct StartCmd {
    /// To whom are we saying hello?
    recipient: Vec<String>,
}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        println!("Hello, {}!", &config.hello.recipient);
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
        if !self.recipient.is_empty() {
            config.hello.recipient = self.recipient.join(" ");
        }

        Ok(config)
    }
}
