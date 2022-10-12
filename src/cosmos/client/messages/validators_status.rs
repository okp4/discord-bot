//! Holds validators state related message and return type

use actix::Message;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{BondStatus, QueryValidatorsResponse};

use crate::cosmos::client::error::Error;

/// Result of a get validators status
pub type GetValidatorsStatusResult = Result<QueryValidatorsResponse, Error>;

/// Get validators status message
#[derive(Message)]
#[rtype(result = "GetValidatorsStatusResult")]
pub struct GetValidatorsStatus {
    /// status has to be specified
    pub status: BondStatus
}
