//! DiscordBot


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

pub mod application;
pub mod chain;
pub mod commands;
pub mod components;
pub mod config;
pub mod discord;
pub mod error;
pub mod prelude;
