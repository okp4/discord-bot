//! Trigger transaction handler

use crate::cosmos::client::messages::broadcast_tx::{BroadcastTx, BroadcastTxResult};
use crate::cosmos::client::messages::get_account::{GetAccount, GetAccountResult};
use crate::cosmos::tx::error::Error;
use crate::cosmos::tx::messages::response::TxResult;
use crate::cosmos::tx::messages::trigger::{TriggerTx, TriggerTxResult};
use crate::cosmos::tx::TxHandler;
use actix::dev::ToEnvelope;
use actix::{Actor, ActorFutureExt, Handler, MailboxError, ResponseActFuture, WrapFuture};
use cosmrs::tx::{Body, Msg};
use serenity::model::user::User;
use std::cmp::min;
use tracing::info;
use tracing::log::error;

impl<T, R> Handler<TriggerTx> for TxHandler<T, R>
where
    T: Msg + Unpin + 'static,
    R: Actor + Handler<TxResult>,
    R::Context: ToEnvelope<R, TxResult>,
{
    type Result = ResponseActFuture<Self, TriggerTxResult>;

    fn handle(&mut self, msg: TriggerTx, _ctx: &mut Self::Context) -> Self::Result {
        let Ok(mut msgs_queue) = self.msgs_queue.lock() else {
            error!("❌ Failed lock msgs queue, request fund couldn't be registered.");
            return Box::pin(async {}.into_actor(self));
        };
        let msgs_queue_len = msgs_queue.len();

        if msgs_queue.is_empty() {
            info!("🥹 No message to submit");
            return Box::pin(async {}.into_actor(self));
        }

        let (subscribers, msgs): (Vec<User>, Vec<T>) = msgs_queue
            .drain(..min(msgs_queue_len, self.max_msg))
            .map(|it| (it.0, it.1))
            .unzip();

        let grpc_client = self.grpc_client.clone();
        let sender_address = self.sender.address.to_string();
        let response_handler = self.response_handler.clone();
        Box::pin(
            async move {
                let result: Result<GetAccountResult, MailboxError> = grpc_client
                    .send(GetAccount {
                        addr: sender_address,
                    })
                    .await;
                (result, msg, grpc_client)
            }
            .into_actor(self)
            .map(move |(res, message, grpc_client), act, _| {
                let body = Body::new(
                    msgs.iter()
                        .map(|msg| msg.to_any().unwrap())
                        .collect::<Vec<_>>(),
                    message.memo,
                    0u16,
                );

                let sign_tx = res
                    .map_err(Error::from)
                    .and_then(|value| value.map_err(Error::from))
                    .and_then(|account| act.sign_tx(&body, account, act.fee.clone()));
                (sign_tx, grpc_client)
            })
            .then(|(sign_tx, grpc_client), act, _| {
                async move {
                    let result: Result<BroadcastTxResult, Error> = match sign_tx {
                        Ok(tx_bytes) => {
                            info!("🔥 Broadcast transaction");
                            grpc_client
                                .send(BroadcastTx { tx: tx_bytes })
                                .await
                                .map_err(Error::from)
                        }
                        Err(why) => {
                            error!("❌ Failed sign transaction: {}", why);
                            Err(why)
                        }
                    };
                    result
                }
                .into_actor(act)
            })
            .map(move |tx_result, _, _| match response_handler {
                Some(r) => r.do_send(TxResult {
                    result: tx_result.and_then(|i| i.map_err(Error::from)),
                    subscribers,
                }),
                None => info!("📩 Transaction broadcasted"),
            }),
        )
    }
}
