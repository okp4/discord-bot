use actix::Message;
use cosmos_sdk_proto::cosmos::staking::v1beta1::BondStatus;

pub type GetStateResult = ();

/// Empty message
#[derive(Message)]
#[rtype(result = "GetStateResult")]
pub struct GetStateMessage {
    /// Bond status
    pub bond_status: BondStatus,

    /// Pagination key
    pub pagination_key: Option<Vec<u8>>,
}
