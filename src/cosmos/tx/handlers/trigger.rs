//! Trigger transaction handler

use crate::cosmos::client::messages::broadcast_tx::BroadcastTx;
use crate::cosmos::client::messages::get_account::{GetAccount, GetAccountResult};
use crate::cosmos::tx::error::Error::AccountNotFound;
use crate::cosmos::tx::messages::trigger::{TriggerTx, TriggerTxResult};
use crate::cosmos::tx::TxHandler;
use actix::{ActorFutureExt, Handler, MailboxError, ResponseActFuture, WrapFuture};
use cosmrs::tx::{Body, Fee, Msg};
use cosmrs::Coin;
use tracing::info;
use tracing::log::error;

impl<T> Handler<TriggerTx> for TxHandler<T>
where
    T: Msg + Unpin + 'static,
{
    type Result = ResponseActFuture<Self, TriggerTxResult>;

    fn handle(&mut self, msg: TriggerTx, _ctx: &mut Self::Context) -> Self::Result {
        if self.msgs.is_empty() {
            info!("🥹 No message to submit");
            return Box::pin(async {}.into_actor(self));
        }

        let msgs = self.msgs.clone();
        self.msgs.clear();

        let grpc_client = self.grpc_client.clone();
        let sender_address = self.sender.address.to_string();
        Box::pin(
            async move {
                let result: Result<GetAccountResult, MailboxError> = grpc_client
                    .send(GetAccount {
                        addr: sender_address,
                    })
                    .await;
                return (result, msg);
            }
            .into_actor(self)
            .map(move |(res, message), act, _| {
                let body = Body::new(
                    msgs.iter()
                        .map(|msg| msg.to_any().unwrap())
                        .collect::<Vec<_>>(),
                    message.memo,
                    0u16,
                );

                match res
                    .map_err(|_| AccountNotFound)
                    .and_then(|value| value.map_err(|_| AccountNotFound))
                    .and_then(|account| {
                        let fee = Fee {
                            amount: vec![Coin {
                                denom: "uknow".parse().unwrap(),
                                amount: 0,
                            }], // TODO: Get this from config.
                            gas_limit: message.gas_limit.into(),
                            payer: None,
                            granter: None,
                        };
                        act.sign_tx(&body, account, fee)
                    }) {
                    Ok(tx_bytes) => {
                        info!("🔥 Trigger transaction");
                        act.grpc_client.do_send(BroadcastTx { tx: tx_bytes })
                    }
                    Err(why) => error!("❌ Failed sign transaction: {}", why),
                }
            }),
        )
    }
}
