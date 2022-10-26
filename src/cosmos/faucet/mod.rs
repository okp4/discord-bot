//! Holds all the faucet actor configuration

mod actor;
pub mod discord_message;
mod handlers;
pub mod messages;

use crate::cosmos::tx::TxHandler;
use crate::discord::discord_client::DiscordActor;
use actix::Addr;
use cosmrs::bank::MsgSend;
use cosmrs::{AccountId, Coin};

/// Represent a faucet actor allowing send defined amount to a recipient.
#[derive(Clone, Debug)]
pub struct Faucet {
    /// Faucet sender account
    pub sender: AccountId,
    /// Transaction amount
    pub amount: Coin,
    /// Transaction handler client address to send transaction
    pub tx_handler: Addr<TxHandler<MsgSend, Self>>,
    /// Discord client that will be used to trigger transaction result
    pub discord_client: Addr<DiscordActor>,
    /// The discord channel id where the transaction result should be posted
    pub channel_id: u64,
    /// Set the explorer url template to add link on the discord message.
    pub explorer_url: Option<String>,
}
