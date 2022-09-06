//! Trigger transaction handler

use crate::cosmos::client::messages::broadcast_tx::{BroadcastTx, BroadcastTxResult};
use crate::cosmos::client::messages::get_account::{GetAccount, GetAccountResult};
use crate::cosmos::tx::error::Error;
use crate::cosmos::tx::messages::trigger::{TriggerTx, TriggerTxResult};
use crate::cosmos::tx::TxHandler;
use actix::{ActorFutureExt, Handler, MailboxError, ResponseActFuture, WrapFuture};
use cosmrs::tx::{Body, Fee, Msg};
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
        let subscribers = self.subscribers.clone();
        self.msgs.clear();
        self.subscribers.clear();

        let grpc_client = self.grpc_client.clone();
        let second_grpc_client = self.grpc_client.clone(); // TODO: remove this 💩
        let sender_address = self.sender.address.to_string();
        Box::pin(
            async move {
                let result: Result<GetAccountResult, MailboxError> = grpc_client
                    .send(GetAccount {
                        addr: sender_address,
                    })
                    .await;
                (result, msg)
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

                res.map_err(Error::from)
                    .and_then(|value| value.map_err(Error::from))
                    .and_then(|account| {
                        let fee = Fee {
                            amount: vec![act.fee_amount.clone()],
                            gas_limit: message.gas_limit.into(),
                            payer: None,
                            granter: None,
                        };
                        act.sign_tx(&body, account, fee)
                    })
            })
            .then(|sign_tx, act, _| {
                async move {
                    let result: Result<BroadcastTxResult, Error> = match sign_tx {
                        Ok(tx_bytes) => {
                            info!("🔥 Broadcast transaction");
                            second_grpc_client
                                .send(BroadcastTx {
                                    tx: tx_bytes,
                                    subscribers,
                                })
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
            .map(|tx_result, _, _| {
                match tx_result.and_then(|i| i.map_err(Error::from)) {
                    Ok(tx_response) => info!(
                        "Transaction successfuylly broadcasted : {}",
                        tx_response.txhash
                    ),
                    Err(why) => error!("❌ Failed sign transaction {}", why),
                }
            }),
        )
    }
}
