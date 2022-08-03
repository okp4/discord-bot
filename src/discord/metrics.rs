//! Metrics constants for the discord client.

/// The metric name for the discord_interactions_total metric.
pub static METRIC_DISCORD_INTERACTIONS_TOTAL: &str = "discord_interactions_total";
/// The metric name for the discord_interactions_duration metric.
pub static METRIC_DISCORD_INTERACTIONS_DURATION: &str = "discord_interactions_duration";

/// The label name for "interaction".
pub static LABEL_NAME_INTERACTION: &str = "interaction";

/// The label name for "command".
pub static LABEL_NAME_COMMAND: &str = "command";

/// The default label value to use when a command is unknown (unparseable).
pub static LABEL_VALUE_COMMAND_UNKNOWN: &str = "???";
