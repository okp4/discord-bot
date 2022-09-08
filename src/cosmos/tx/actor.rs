use crate::cosmos::tx::messages::trigger::TriggerTx;
use crate::cosmos::tx::TxHandler;
use actix::{Actor, AsyncContext, Context};
use cosmrs::tx::Msg;
use tracing::info;

impl<T> Actor for TxHandler<T>
where
    T: Msg + Unpin + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("📣 TxHandler started...");
        ctx.run_interval(self.batch_window, |act, ctx| {
            ctx.address().do_send(TriggerTx {
                memo: act.memo.clone(),
                fee: act.fee.clone(),
            })
        });
    }
}
