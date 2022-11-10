//! Register transaction handler

use crate::cosmos::tx::messages::register_msg::{RegisterMsg, RegisterMsgResult};
use crate::cosmos::tx::messages::response::TxResult;
use crate::cosmos::tx::TxHandler;
use actix::dev::ToEnvelope;
use actix::{Actor, Handler};
use cosmrs::tx::Msg;
use tracing::info;

impl<T, R> Handler<RegisterMsg<T>> for TxHandler<T, R>
where
    T: Msg + Unpin + 'static + PartialEq,
    R: Actor + Handler<TxResult>,
    R::Context: ToEnvelope<R, TxResult>,
{
    type Result = RegisterMsgResult;

    fn handle(&mut self, msg: RegisterMsg<T>, _: &mut Self::Context) -> Self::Result {
        let mut msgs = self.msgs_queue.lock().unwrap();
        if msgs.iter().find(|f| f.0.id == msg.subscriber.id) == None {
            info!("🤑 Register transaction for {}", msg.subscriber.name);
            msgs.push_back((msg.subscriber, msg.msg));
        }
        else {
            info!("👮‍ The user {} already register transaction, skip this one.", msg.subscriber.name);
        }
    }
}
