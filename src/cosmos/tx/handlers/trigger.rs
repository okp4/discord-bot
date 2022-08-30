//! Trigger transaction handler

use crate::cosmos::tx::messages::trigger::{TriggerTx, TriggerTxResult};
use crate::cosmos::tx::tx::TxHandler;
use actix::Handler;
use cosmrs::auth::BaseAccount;
use cosmrs::tx::{Body, Fee, Msg};
use tracing::info;
use tracing::log::error;

impl Handler<TriggerTx> for TxHandler {
    type Result = TriggerTxResult;

    fn handle(&mut self, msg: TriggerTx, _: &mut Self::Context) -> Self::Result {
        if self.msgs.len() == 0 {
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
            amount: self
                .msgs
                .iter()
                .map(|msg| msg.amount.clone())
                .flatten()
                .collect::<Vec<_>>(),
            gas_limit: msg.gas_limit.into(),
            payer: None,
            granter: None,
        };
        match &self
            .signing_key()
            .and_then(|key| self.sign_tx(&body, account, key, fee))
        {
            Ok(_) => info!("🔥 Trigger transaction"),
            Err(why) => error!("❌ Failed sign transaction: {}", why),
        }

        // TODO: Broadcast tx

        self.msgs.clear();
    }
}
