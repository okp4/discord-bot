//! Register transaction message

use actix::Message;
use cosmrs::bank::MsgSend;

/// Result of a register tx message.
pub type RegisterTxResult = ();

/// Register transaction actor message.
#[derive(Message)]
#[rtype(result = "RegisterTxResult")]
pub struct RegisterTx {
    /// Contains the messages to embed in the transaction.
    pub msg: MsgSend,
}

impl RegisterTx {
    /// Create a new RegisterTx.
    pub fn new(msg: MsgSend) -> Self {
        Self { msg }
    }
}
