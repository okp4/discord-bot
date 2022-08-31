//! Holds all the faucet actor configuration

mod actor;
mod handlers;
mod messages;

use crate::cosmos::client::account::Account;
use crate::cosmos::tx::TxHandler;
use actix::Addr;
use cosmrs::Coin;

/// Represent a faucet actor allowing send defined amount to a recipient.
pub struct Faucet {
    /// Faucet sender account
    pub sender: Account,
    /// Transaction amount
    pub amount: Coin,
    /// Transaction handler client address to send transaction
    pub tx_handler: Addr<TxHandler>,
}
