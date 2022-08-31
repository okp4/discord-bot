//! Holds transaction broadcast related types

use actix::Message;

/// Result of a transaction broadcast message
pub type BroadcastTxResult = ();

/// Transaction broadcast message
#[derive(Message)]
#[rtype(result = "BroadcastTxResult")]
pub struct BroadcastTx {
    pub tx: Vec<u8>,
}
