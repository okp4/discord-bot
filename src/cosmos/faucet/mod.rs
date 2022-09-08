//! Holds all the faucet actor configuration

mod actor;
pub mod discord_message;
mod handlers;
pub mod messages;

use crate::cosmos::tx::TxHandler;
use actix::{Addr, Handler};
use cosmrs::bank::MsgSend;
use cosmrs::{AccountId, Coin};
use crate::cosmos::tx::messages::register::RegisterMsg;

/// Represent a faucet actor allowing send defined amount to a recipient.
#[derive(Clone, Debug)]
pub struct Faucet {
    /// Faucet sender account
    pub sender: AccountId,
    /// Transaction amount
    pub amount: Coin,
    /// Transaction handler client address to send transaction
    pub tx_handler: Addr<TxHandler<MsgSend, Faucet>>,
}


