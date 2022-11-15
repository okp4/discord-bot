use crate::cosmos::faucet::messages::request_funds::{RequestFunds, RequestFundsResult};
use crate::cosmos::faucet::Faucet;
use crate::cosmos::tx::error::Error::Mailbox;
use crate::cosmos::tx::messages::register_msg::{RegisterMsg, RegisterMsgResult};
use actix::{Handler, MailboxError, ResponseFuture};
use cosmrs::bank::MsgSend;
use tracing::info;

impl Handler<RequestFunds> for Faucet {
    type Result = ResponseFuture<RequestFundsResult>;

    fn handle(&mut self, msg: RequestFunds, _: &mut Self::Context) -> Self::Result {
        let msg_send = MsgSend {
            from_address: self.sender.clone(),
            to_address: msg.address.clone(),
            amount: vec![self.amount.clone()],
        };

        info!("✍️ Register request funds for {}", msg.address);

        let tx_handler = self.tx_handler.clone();
        Box::pin(async move {
            let result: Result<RegisterMsgResult, MailboxError> = tx_handler
                .send(RegisterMsg::new(msg_send, msg.requester))
                .await;

            result.map_err(|e| Mailbox(e.to_string())).and_then(|r| r)
        })
    }
}
