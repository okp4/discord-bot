//! Register transaction handler

use crate::cosmos::tx::messages::register_msg::{RegisterMsg, RegisterMsgResult};
use crate::cosmos::tx::messages::response::TxResponse;
use crate::cosmos::tx::TxHandler;
use actix::dev::ToEnvelope;
use actix::{Actor, Handler};
use cosmrs::tx::Msg;

impl<T, R> Handler<RegisterMsg<T>> for TxHandler<T, R>
where
    T: Msg + Unpin + 'static,
    R: Actor + Handler<TxResponse>,
    R::Context: ToEnvelope<R, TxResponse>,
{
    type Result = RegisterMsgResult;

    fn handle(&mut self, msg: RegisterMsg<T>, _: &mut Self::Context) -> Self::Result {
        if msg.subscriber.is_some() {
            self.subscribers.push(msg.subscriber.unwrap())
        }
        self.msgs.push(msg.msg);
    }
}