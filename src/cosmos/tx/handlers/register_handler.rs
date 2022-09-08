//! Register transaction handler

use crate::cosmos::tx::messages::register_handler::{
    RegisterResponseHandler, RegisterResponseHandlerResult,
};
use crate::cosmos::tx::messages::response::TxResponse;
use crate::cosmos::tx::TxHandler;
use actix::dev::ToEnvelope;
use actix::{Actor, Handler};
use cosmrs::tx::Msg;
use tracing::info;

impl<T, R> Handler<RegisterResponseHandler<R>> for TxHandler<T, R>
where
    T: Msg + Unpin + 'static,
    R: Actor + Handler<TxResponse>,
    R::Context: ToEnvelope<R, TxResponse>,
{
    type Result = RegisterResponseHandlerResult;

    fn handle(&mut self, msg: RegisterResponseHandler<R>, _: &mut Self::Context) -> Self::Result {
        info!("📣 Register response handler...");
        self.response_handler = Some(msg.handler);
    }
}
