//! Trigger transaction message

use actix::Message;

/// Trigger transaction actor message result.
pub type TriggerTxResult = ();

/// Trigger transaction actor message.
#[derive(Message)]
#[rtype(result = "TriggerTxResult")]
pub struct TriggerTx {
    /// The memo field content of the transaction.
    pub memo: String,

    /// gas limit of the transaction.
    pub gas_limit: u64,
}
