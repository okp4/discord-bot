use crate::cosmos::faucet::Faucet;
use crate::cosmos::tx::messages::response::{TxResponse, TxResponseResult};
use actix::Handler;
use tracing::info;
use crate::cosmos::faucet::discord_message::FaucetTransactionMessage;
use crate::cosmos::tx::discord_message::TransactionDiscordMessage;
use crate::discord::discord_client::messages::send_msg::SendMessage;

impl Handler<TxResponse> for Faucet {
    type Result = TxResponseResult;

    fn handle(&mut self, msg: TxResponse, _: &mut Self::Context) -> Self::Result {
        info!("📝 Receive transaction result on faucet.");
        let message = FaucetTransactionMessage::build_message(msg.response, msg.subscribers, self.channel_id);
        self.discord_client.do_send(SendMessage { message });
    }
}
