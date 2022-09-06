//! Holds transaction broadcast related types

use crate::cosmos::client::error::Error;
use actix::Message;
use cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use serenity::model::user::User;

/// Result of a transaction broadcast message
pub type BroadcastTxResult = Result<TxResponse, Error>;

/// Transaction broadcast message
#[derive(Message)]
#[rtype(result = "BroadcastTxResult")]
pub struct BroadcastTx {
    /// Transaction to broadcast (binary array)
    pub tx: Vec<u8>,
    /// Subcribers of the transactions
    pub subscribers: Vec<User>,
}
