use actix::Handler;
use tonic::transport::Channel;

use crate::{messages::request::{RequestTx, RequestTxResult}, grpc::client::Client};

impl Handler<RequestTx> for Client<'static, Channel> {
    type Result = RequestTxResult;

    fn handle(&mut self, msg: RequestTx, _ctx: &mut Self::Context) -> Self::Result {
			self.txs.push(&msg.0);
    }
}