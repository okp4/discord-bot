use crate::cosmos::faucet::messages::request_funds::{RequestFunds, RequestFundsResult};
use crate::cosmos::faucet::Faucet;
use crate::cosmos::tx::messages::register::RegisterTx;
use actix::Handler;
use cosmrs::bank::MsgSend;
use tracing::info;

impl Handler<RequestFunds> for Faucet {
    type Result = RequestFundsResult;

    fn handle(&mut self, msg: RequestFunds, _: &mut Self::Context) -> Self::Result {
        let msg_send = MsgSend {
            from_address: self.sender.clone(),
            to_address: msg.address.clone(),
            amount: vec![self.amount.clone()],
        };

        self.tx_handler
            .do_send(RegisterTx::new(msg_send, Some(msg.requester)));

        info!("✍️  Register request funds for {}", msg.address);
    }
}
