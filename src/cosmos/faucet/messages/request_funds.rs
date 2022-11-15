//! Holds Request funds messages
use crate::cosmos::tx::error::Error;
use actix::Message;
use cosmrs::AccountId;
use serenity::model::user::User;

/// Represent the RequestFunds message result
pub type RequestFundsResult = Result<(), Error>;

/// Request funds from the faucet message
#[derive(Message)]
#[rtype(result = "RequestFundsResult")]
pub struct RequestFunds {
    /// Address which will receive funds
    pub address: AccountId,

    /// Discord user that request funds
    pub requester: User,
}
