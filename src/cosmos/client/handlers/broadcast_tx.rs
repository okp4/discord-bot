use actix::{ContextFutureSpawner, Handler, WrapFuture};
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest;
use tonic::transport::Channel;
use tracing::info;

use crate::cosmos::client::{
    messages::broadcast_tx::{BroadcastTx, BroadcastTxResult},
    Client,
};

impl Handler<BroadcastTx> for Client<Channel> {
    type Result = BroadcastTxResult;

    fn handle(&mut self, msg: BroadcastTx, ctx: &mut Self::Context) -> Self::Result {
        let mut tx_client = self.clone().tx();
        async move {
            let _ = tx_client
                .broadcast_tx(tonic::Request::new(BroadcastTxRequest {
                    tx_bytes: msg.tx,
                    mode: 2,
                }))
                .await;
            info!("✉️ Broadcast transaction•s")
        }
        .into_actor(self)
        .wait(ctx);
    }
}
