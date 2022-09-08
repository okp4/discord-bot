//! Register transaction handler

use crate::cosmos::tx::discord_message::TransactionDiscordMessage;
use crate::cosmos::tx::messages::register::{RegisterMsg, RegisterMsgResult};
use crate::cosmos::tx::TxHandler;
use crate::discord::discord_client::message::DiscordMessage;
use actix::Handler;
use cosmrs::tx::Msg;

impl<T, M> Handler<RegisterMsg<T>> for TxHandler<T, M>
where
    T: Msg + Unpin + 'static,
    M: TransactionDiscordMessage + DiscordMessage + Unpin + Send + 'static,
{
    type Result = RegisterMsgResult;

    fn handle(&mut self, msg: RegisterMsg<T>, _: &mut Self::Context) -> Self::Result {
        if msg.subscriber.is_some() {
            self.subscribers.push(msg.subscriber.unwrap())
        }
        self.msgs.push(msg.msg);
    }
}
