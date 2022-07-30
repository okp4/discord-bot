//! DiscordBot

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

pub mod application;
pub mod commands;
pub mod components;
pub mod config;
pub mod discord;
pub mod error;
pub mod prelude;
