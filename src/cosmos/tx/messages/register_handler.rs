//! Message to register response handler.

use crate::cosmos::tx::messages::response::TxResult;
use actix::dev::ToEnvelope;
use actix::{Actor, Addr, Handler, Message};

/// Result returned by the register response handler message.
pub type RegisterResponseHandlerResult = ();

/// Message to register the response handler address
#[derive(Message)]
#[rtype(result = "RegisterResponseHandlerResult")]
pub struct RegisterResponseHandler<R>
where
    R: Actor + Handler<TxResult>,
    R::Context: ToEnvelope<R, TxResult>,
{
    /// Address of actor handler that will receive transaction result.
    pub handler: Addr<R>,
}
