use actix::Handler;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest;
use tonic::transport::Channel;

use crate::cosmos::client::{
    messages::broadcast_tx::{BroadcastTx, BroadcastTxResult},
    Client,
};

impl Handler<BroadcastTx> for Client<Channel> {
    type Result = BroadcastTxResult;

    fn handle(&mut self, msg: BroadcastTx, _ctx: &mut Self::Context) -> Self::Result {
        let _ = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(async move {
                self.clone()
                    .tx()
                    .broadcast_tx(tonic::Request::new(BroadcastTxRequest {
                        tx_bytes: msg.tx,
                        mode: 2,
                    }))
                    .await
            });
        // TODO: notify discord when the tx is broadcasted
    }
}
