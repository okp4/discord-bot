//! Trigger transaction handler

use crate::cosmos::client::messages::broadcast_tx::BroadcastTx;
use crate::cosmos::client::messages::get_account::GetAccount;
use crate::cosmos::tx::error::Error;
use crate::cosmos::tx::error::Error::AccountNotFound;
use crate::cosmos::tx::messages::trigger::{TriggerTx, TriggerTxResult};
use crate::cosmos::tx::TxHandler;
use actix::{AsyncContext, Handler, WrapFuture};
use cosmrs::tx::{Body, Fee, Msg};
use cosmrs::Coin;
use std::sync::mpsc;
use tracing::info;
use tracing::log::error;

impl<T> Handler<TriggerTx> for TxHandler<T>
where
    T: Msg + Unpin + 'static,
{
    type Result = TriggerTxResult;

    fn handle(&mut self, msg: TriggerTx, _ctx: &mut Self::Context) -> Self::Result {
        if self.msgs.is_empty() {
            info!("🥹 No message to submit");
            return;
        }

        let (tx, rx) = mpsc::channel();
        let grpc_client = self.grpc_client.clone();
        let sender_address = self.sender.address.to_string();
        _ctx.wait(
            async move {
                let result = grpc_client
                    .send(GetAccount {
                        addr: sender_address,
                    })
                    .await;

                let account = match result {
                    Ok(fut) => fut.map_err(Error::from),
                    Err(why) => {
                        error!("❌ Failed fetch grpc actor {}", why);
                        Err(AccountNotFound)
                    }
                };

                tx.send(account).unwrap();
            }
            .into_actor(self),
        );

        match rx
            .recv().unwrap()
            .map_err(|_| AccountNotFound)
            .and_then(|account| {
                let body = Body::new(
                    self.msgs
                        .iter()
                        .map(|msg| msg.to_any().unwrap())
                        .collect::<Vec<_>>(),
                    msg.memo,
                    0u16,
                );

                let fee = Fee {
                    amount: vec![Coin {
                        denom: "uknow".parse().unwrap(),
                        amount: 0,
                    }], // TODO: Get this from config.
                    gas_limit: msg.gas_limit.into(),
                    payer: None,
                    granter: None,
                };
                self.sign_tx(&body, account, fee)
            }) {
            Ok(tx_bytes) => {
                info!("🔥 Trigger transaction");
                self.grpc_client.do_send(BroadcastTx { tx: tx_bytes })
            }
            Err(why) => error!("❌ Failed sign transaction: {}", why),
        }

        self.msgs.clear();
    }
}
