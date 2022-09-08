use crate::cosmos::faucet::messages::request_funds::{RequestFunds, RequestFundsResult};
use crate::cosmos::faucet::Faucet;
use crate::cosmos::tx::messages::register::RegisterMsg;
use actix::Handler;
use cosmrs::bank::MsgSend;
use tracing::info;
use crate::cosmos::tx::messages::response::{TxResponse, TxResponseResult};

impl Handler<TxResponse> for Faucet {
    type Result = TxResponseResult;

    fn handle(&mut self, msg: TxResponse, _: &mut Self::Context) -> Self::Result {
        info!("✍️  📝 Receive transaction result on faucet");
    }
}
