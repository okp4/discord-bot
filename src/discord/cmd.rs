//! Holds some types for discord slash commands.
use crate::discord::error::Error;
use strum_macros::Display;
use strum_macros::EnumString;

/// Type which specifies the expected result of the command execution.
pub type CommandExecutionResult = Result<String, Error>;

/// The different supported commands.
#[derive(Display, EnumString)]
pub enum DiscordCommand {
    /// The ping command.
    #[strum(serialize = "ping")]
    Ping,
}
