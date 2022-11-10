//! Trigger transaction handler

use crate::cosmos::client::messages::broadcast_tx::{BroadcastTx, BroadcastTxResult};
use crate::cosmos::client::messages::get_account::{GetAccount, GetAccountResult};
use crate::cosmos::tx::discord_message::TransactionDiscordMessage;
use crate::cosmos::tx::error::Error;
use crate::cosmos::tx::messages::trigger::{TriggerTx, TriggerTxResult};
use crate::cosmos::tx::TxHandler;
use crate::discord::discord_client::message::DiscordMessage;
use crate::discord::discord_client::messages::send_msg::SendMessage;
use actix::{ActorFutureExt, Handler, MailboxError, ResponseActFuture, WrapFuture};
use cosmrs::tx::{Body, Msg};
use tracing::info;
use tracing::log::error;

impl<T, M> Handler<TriggerTx> for TxHandler<T, M>
where
    T: Msg + Unpin + 'static,
    M: TransactionDiscordMessage + DiscordMessage + Unpin + Send + 'static,
{
    type Result = ResponseActFuture<Self, TriggerTxResult>;

    fn handle(&mut self, msg: TriggerTx, _ctx: &mut Self::Context) -> Self::Result {
        if self.msgs.is_empty() {
            info!("🥹 No message to submit");
            return Box::pin(async {}.into_actor(self));
        }

        let msgs = self.msgs.clone();
        let subscribers = self.subscribers.clone();
        self.msgs.clear();
        self.subscribers.clear();

        let grpc_client = self.grpc_client.clone();
        let sender_address = self.sender.address.to_string();
        let discord_client = self.discord_client.clone();
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
            .map(move |tx_result, act, _| {
                info!("📩 Transaction broadcasted");
                let discord_message = M::build_message(
                    tx_result.and_then(|i| i.map_err(Error::from)),
                    subscribers,
                    act.channel_id.unwrap(),
                );
                discord_client.do_send(SendMessage {
                    message: discord_message,
                });
            }),
        )
    }
}
