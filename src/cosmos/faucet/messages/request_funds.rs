//! Holds Request funds messages
use actix::Message;
use cosmrs::AccountId;

/// Represent the RequestFunds message result
pub type RequestFundsResult = ();

/// Request funds from the faucet message
#[derive(Message)]
#[rtype(result = "RequestFundsResult")]
pub struct RequestFunds {
    /// Address which will receive funds
    pub address: AccountId,
}
