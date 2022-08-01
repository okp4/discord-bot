//! Main entry point for DiscordBot

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use discord_bot::application::APP;

/// Boot DiscordBot
fn main() {
    abscissa_core::boot(&APP);
}
