//! Trigger transaction message

use actix::Message;
use cosmrs::tx::Fee;

/// Trigger transaction actor message result.
pub type TriggerTxResult = ();

/// Trigger transaction actor message.
#[derive(Message)]
#[rtype(result = "TriggerTxResult")]
pub struct TriggerTx {
    /// The memo field content of the transaction.
    pub memo: String,

    /// Transaction fee
    pub fee: Fee,
}
