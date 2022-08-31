//! Holds get account related message and return type

use actix::Message;
use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;

use crate::error::Error;

/// Result of a get account request
pub type GetAccountResult = Result<BaseAccount, Error>;

/// Get account request message
#[derive(Message)]
#[rtype(result = "GetAccountResult")]
pub struct GetAccount {
    /// Account address
    pub addr: String,
}
