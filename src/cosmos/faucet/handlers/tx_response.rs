use crate::cosmos::faucet::Faucet;
use crate::cosmos::tx::messages::response::{TxResponse, TxResponseResult};
use actix::Handler;
use tracing::info;

impl Handler<TxResponse> for Faucet {
    type Result = TxResponseResult;

    fn handle(&mut self, _: TxResponse, _: &mut Self::Context) -> Self::Result {
        info!("✍️  📝 Receive transaction result on faucet");
    }
}
