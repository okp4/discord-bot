//! Register transaction handler

use crate::cosmos::tx::messages::register::{RegisterTx, RegisterTxResult};
use crate::cosmos::tx::tx::TxHandler;
use actix::Handler;

impl Handler<RegisterTx> for TxHandler {
    type Result = RegisterTxResult;

    fn handle(&mut self, msg: RegisterTx, _: &mut Self::Context) -> Self::Result {
        self.msgs.push(msg.msg);
    }
}
