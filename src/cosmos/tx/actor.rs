use crate::cosmos::tx::messages::trigger::TriggerTx;
use crate::cosmos::tx::TxHandler;
use actix::{Actor, AsyncContext, Context};
use std::time::Duration;
use tracing::info;

impl Actor for TxHandler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("📣 TxHandler started...");
        ctx.run_interval(Duration::new(8, 0), |act, ctx| {
            ctx.address().do_send(TriggerTx {
                memo: act.memo.clone(),
                gas_limit: act.gas_limit,
            })
        });
    }
}
