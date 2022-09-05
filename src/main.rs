//! Main entry point for DiscordBot

#![forbid(unsafe_code)]
#![deny(
    warnings,
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_import_braces,
    unused_qualifications,
    unused_qualifications
)]

pub mod cli;
pub mod cosmos;
pub mod discord_client;
pub mod discord_server;
pub mod error;

use crate::cli::prelude::*;

/// Boot DiscordBot
fn main() {
    abscissa_core::boot(&APP);
}
