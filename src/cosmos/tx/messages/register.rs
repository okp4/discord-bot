//! Register transaction message

use actix::Message;
use cosmrs::tx::Msg;
use serenity::model::user::User;

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
    pub(crate) msg: T,

    /// Transaction subscriber
    pub(crate) subscriber: Option<User>,
}

impl<T> RegisterTx<T>
where
    T: Msg,
{
    /// Create a new RegisterTx.
    pub fn new(msg: T, subscriber: Option<User>) -> Self
    where
        T: Msg,
    {
        Self { msg, subscriber }
    }
}
