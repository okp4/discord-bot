//! Holds transaction request related types

use actix::Message;

/// Result of a transaction request message
pub type SendTxResult = ();

/// Transaction request message
#[derive(Message)]
#[rtype(result = "SendTxResult")]
pub struct SendTx();
