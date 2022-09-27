//! Handler of transaction response.

use crate::cosmos::tx::error::Error;
use actix::Message;
use cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use serenity::model::user::User;

/// Result returned by the TxResponse message.
pub type TxResponseResult = ();

/// Message used to receive and handle all transaction response after triggerred.
#[derive(Message)]
#[rtype(result = "TxResponseResult")]
pub struct TxResult {
    /// Contains the result of transaction
    pub result: Result<TxResponse, Error>,
    /// List of the subscriber concerned by the transactions
    pub subscribers: Vec<User>,
}
