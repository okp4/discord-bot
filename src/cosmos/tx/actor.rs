use crate::cosmos::tx::messages::response::TxResponse;
use crate::cosmos::tx::messages::trigger::TriggerTx;
use crate::cosmos::tx::TxHandler;
use actix::dev::ToEnvelope;
use actix::{Actor, AsyncContext, Context, Handler};
use cosmrs::tx::Msg;
use tracing::info;

impl<T, R> Actor for TxHandler<T, R>
where
    T: Msg + Unpin + 'static,
    R: Actor + Handler<TxResponse>,
    R::Context: ToEnvelope<R, TxResponse>,
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
