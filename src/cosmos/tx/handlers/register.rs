use actix::Handler;
use crate::cosmos::tx::messages::register::{RegisterTx, RegisterTxResult};
use crate::cosmos::tx::tx::TxHandler;

impl Handler<RegisterTx> for TxHandler {
    type Result = RegisterTxResult;

    fn handle(&mut self, msg: RegisterTx, ctx: &mut Self::Context) -> Self::Result {
        self.msgs.push(msg.msg);
    }
}
