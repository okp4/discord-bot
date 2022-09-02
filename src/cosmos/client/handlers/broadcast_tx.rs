use actix::{ActorFutureExt, Handler, ResponseActFuture, WrapFuture};
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest;
use tonic::transport::Channel;
use tracing::info;

use crate::cosmos::client::{
    messages::broadcast_tx::{BroadcastTx, BroadcastTxResult},
    Client,
};

impl Handler<BroadcastTx> for Client<Channel> {
    type Result = ResponseActFuture<Self, BroadcastTxResult>;

    fn handle(&mut self, msg: BroadcastTx, _ctx: &mut Self::Context) -> Self::Result {
        let mut tx_client = self.clone().tx();
        Box::pin(
            async move {
                tx_client
                    .broadcast_tx(tonic::Request::new(BroadcastTxRequest {
                        tx_bytes: msg.tx,
                        mode: 2,
                    }))
                    .await
            }
            .into_actor(self)
            .map(|_, _, _| info!("✉️ Broadcast transaction•s")),
        )
    }
}
