//! Holds all the faucet actor configuration

mod actor;
mod handlers;
pub mod messages;

use crate::cosmos::tx::TxHandler;
use crate::discord::discord_client::message::FaucetTransactionMessage;
use actix::Addr;
use cosmrs::bank::MsgSend;
use cosmrs::{AccountId, Coin};

/// Represent a faucet actor allowing send defined amount to a recipient.
pub struct Faucet {
    /// Faucet sender account
    pub sender: AccountId,
    /// Transaction amount
    pub amount: Coin,
    /// Transaction handler client address to send transaction
    pub tx_handler: Addr<TxHandler<MsgSend, FaucetTransactionMessage>>,
}
