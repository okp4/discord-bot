//! Holds transaction request related types

use actix::Message;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest;
use tonic::Request;

/// Result of a transaction request message
pub type RequestTxResult = ();

/// Transaction request message
#[derive(Message)]
#[rtype(result = "RequestTxResult")]
pub struct RequestTx(pub Request<BroadcastTxRequest>);