//! Register transaction message

use actix::Message;
use cosmrs::tx::Msg;
use serenity::model::user::User;

/// Result of a RegisterMsg message.
pub type RegisterMsgResult = ();

/// Register transaction's message actor message.
#[derive(Message)]
#[rtype(result = "RegisterMsgResult")]
pub struct RegisterMsg<T>
where
    T: Msg,
{
    /// Contains the messages to embed in the transaction.
    pub msg: T,

    /// Transaction subscriber
    pub subscriber: Option<User>,
}

impl<T> RegisterMsg<T>
where
    T: Msg,
{
    /// Create a new RegisterMsg.
    pub fn new(msg: T, subscriber: Option<User>) -> Self
    where
        T: Msg,
    {
        Self { msg, subscriber }
    }
}
