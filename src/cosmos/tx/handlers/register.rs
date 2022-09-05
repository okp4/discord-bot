//! Register transaction handler

use crate::cosmos::tx::messages::register::{RegisterTx, RegisterTxResult};
use crate::cosmos::tx::TxHandler;
use actix::Handler;
use cosmrs::tx::Msg;

impl<T> Handler<RegisterTx<T>> for TxHandler<T>
where
    T: Msg + Unpin + 'static,
{
    type Result = RegisterTxResult;

    fn handle(&mut self, msg: RegisterTx<T>, _: &mut Self::Context) -> Self::Result {
        if msg.subscriber.is_some() {
            self.subscribers.push(msg.subscriber.unwrap())
        }
        self.msgs.push(msg.msg);
    }
}
