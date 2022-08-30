use actix::Handler;
use tonic::transport::Channel;

use crate::cosmos::client::{messages::send_tx::{SendTx, SendTxResult}, Client};

impl Handler<SendTx> for Client<Channel> {
    type Result = SendTxResult;

    fn handle(&mut self, _msg: SendTx, _ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}