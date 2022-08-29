//! DiscordBot Abscissa Application

use abscissa_core::{
    application::{self, AppCell},
    config::{self, CfgCell},
    trace, Application, FrameworkError, StandardPaths,
};
use tracing::info;

use super::{config::DiscordBotConfig, commands::EntryPoint};

/// Application state
pub static APP: AppCell<DiscordBotApp> = AppCell::new();

/// DiscordBot Application
#[derive(Debug)]
pub struct DiscordBotApp {
    /// Application configuration.
    config: CfgCell<DiscordBotConfig>,

    /// Application state.
    state: application::State<Self>,
}

/// Initialize a new application instance.
impl Default for DiscordBotApp {
    fn default() -> Self {
        Self {
            config: CfgCell::default(),
            state: application::State::default(),
        }
    }
}

impl Application for DiscordBotApp {
    /// Entrypoint command for this application.
    type Cmd = EntryPoint;

    /// Application configuration.
    type Cfg = DiscordBotConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> config::Reader<DiscordBotConfig> {
        self.config.read()
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Register all components used by this application.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        use crate::cli::components::metrics::MetricsEndpoint;
        use abscissa_tokio::TokioComponent;

        info!("📌 Registering components...");

        let mut framework_components = self.framework_components(command)?;

        framework_components.push(Box::new(TokioComponent::new()?));
        framework_components.push(Box::new(MetricsEndpoint::new()?));

        self.state.components_mut().register(framework_components)
    }

    /// Post-configuration lifecycle callback.
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        let mut components = self.state.components_mut();
        components.after_config(&config)?;
        self.config.set_once(config);

        Ok(())
    }

    /// Get tracing configuration from command-line options
    fn tracing_config(&self, command: &EntryPoint) -> trace::Config {
        if command.verbose {
            trace::Config::verbose()
        } else {
            trace::Config::default()
        }
    }
}
