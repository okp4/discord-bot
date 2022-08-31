//! Register transaction message

use actix::Message;
use cosmrs::tx::Msg;

/// Result of a register tx message.
pub type RegisterTxResult = ();

/// Register transaction actor message.
#[derive(Message)]
#[rtype(result = "RegisterTxResult")]
pub struct RegisterTx<T>
where
    T: Msg,
{
    /// Contains the messages to embed in the transaction.
    pub msg: T,
}

impl<T> RegisterTx<T>
where
    T: Msg,
{
    /// Create a new RegisterTx.
    pub fn new(msg: T) -> Self
    where
        T: Msg,
    {
        Self { msg }
    }
}
