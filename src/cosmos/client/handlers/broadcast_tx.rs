use abscissa_core::Application;
use actix::{ContextFutureSpawner, Handler, WrapFuture};
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest;
use serenity::prelude::Mentionable;
use tonic::{transport::Channel, Status};
use tracing::{info, log::error};

use crate::{
    cli::prelude::APP,
    cosmos::client::{
        messages::broadcast_tx::{BroadcastTx, BroadcastTxResult},
        Client,
    },
    discord_client::messages::send_msg::SendMessage,
};

impl Handler<BroadcastTx> for Client<Channel> {
    type Result = BroadcastTxResult;

    fn handle(&mut self, msg: BroadcastTx, ctx: &mut Self::Context) -> Self::Result {
        let mut tx_client = self.clone().tx();
        let discord_client = self.addr_discord_client.clone();
        let config = APP.config();

        async move {
            info!("✉️ Broadcast transaction•s");
            let _ = tx_client
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
                .map(|tx_response| {
                    discord_client.do_send(SendMessage {
                        title: String::from("🚀 Transaction broadcasted!"),
                        description: format!(
                            "\t- 🤝 Transaction hash: {}
                            \t- ⚙️ Result code : {}
                            \t- ⛽️ Gas used: {}",
                            tx_response.txhash, tx_response.code, tx_response.gas_used
                        ),
                        content: {
                            let mut str = String::new();
                            for sub in msg.subscribers {
                                str.push_str(
                                    &format_args!("{member} ", member = &sub.mention()).to_string(),
                                );
                            }
                            str
                        },
                        channel_id: config.faucet.channel_id,
                    })
                });
        }
        .into_actor(self)
        .wait(ctx);
    }
}
