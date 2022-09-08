use actix::{Handler, ResponseActFuture, WrapFuture};
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest;
use tonic::{transport::Channel, Status};
use tracing::{info, log::error};

use crate::cosmos::client::error::Error;
use crate::cosmos::client::{
    messages::broadcast_tx::{BroadcastTx, BroadcastTxResult},
    Client,
};

impl Handler<BroadcastTx> for Client<Channel> {
    type Result = ResponseActFuture<Self, BroadcastTxResult>;

    fn handle(&mut self, msg: BroadcastTx, _: &mut Self::Context) -> Self::Result {
        let mut tx_client = self.clone().tx();

        Box::pin(
            async move {
                info!("✉️ Broadcast transaction•s");
                tx_client
                    .broadcast_tx(tonic::Request::new(BroadcastTxRequest {
                        tx_bytes: msg.tx,
                        mode: 1,
                    }))
                    .await
                    .map_err(|err| {
                        error!(
                            "☠ Failed to broadcast transaction: {} {}",
                            err.code(),
                            err.message()
                        );
                        err
                    })
                    .and_then(|res| {
                        res.get_ref()
                            .tx_response
                            .clone()
                            .ok_or_else(|| Status::not_found("No transaction response"))
                    })
                    .map_err(Error::from)
            }
            .into_actor(self),
        )
    }
}
