//! Trigger transaction handler

use crate::cosmos::client::messages::broadcast_tx::BroadcastTx;
use crate::cosmos::tx::messages::trigger::{TriggerTx, TriggerTxResult};
use crate::cosmos::tx::TxHandler;
use actix::Handler;
use cosmrs::auth::BaseAccount;
use cosmrs::tx::{Body, Fee, Msg};
use cosmrs::Coin;
use tracing::info;
use tracing::log::error;

impl<T> Handler<TriggerTx> for TxHandler<T>
where
    T: Msg + Unpin + 'static,
{
    type Result = TriggerTxResult;

    fn handle(&mut self, msg: TriggerTx, _: &mut Self::Context) -> Self::Result {
        if self.msgs.is_empty() {
            info!("🥹 No message to submit");
            return;
        }

        let body = Body::new(
            self.msgs
                .iter()
                .map(|msg| msg.to_any().unwrap())
                .collect::<Vec<_>>(),
            msg.memo,
            0u16,
        );
        let account = BaseAccount {
            address: "fake".parse().unwrap(),
            pubkey: None,
            account_number: 0,
            sequence: 0,
        }; // TODO: Ask grpc actor too get the base account
        let fee = Fee {
            amount: vec![Coin {
                denom: "uknow".parse().unwrap(),
                amount: 0,
            }], // TODO: Get this from config.
            gas_limit: msg.gas_limit.into(),
            payer: None,
            granter: None,
        };
        match self.sign_tx(&body, account, fee) {
            Ok(tx_bytes) => {
                info!("🔥 Trigger transaction");
                self.grpc_client.do_send(BroadcastTx { tx: tx_bytes })
            }
            Err(why) => error!("❌ Failed sign transaction: {}", why),
        }

        self.msgs.clear();
    }
}
