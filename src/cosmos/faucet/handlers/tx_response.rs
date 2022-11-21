use crate::cosmos::faucet::discord_message::FaucetTransactionMessage;
use crate::cosmos::faucet::Faucet;
use crate::cosmos::tx::discord_message::TransactionDiscordMessage;
use crate::cosmos::tx::messages::response::{TxResponseResult, TxResult};
use crate::discord::discord_client::messages::send_msg::SendMessage;
use actix::Handler;
use tracing::info;

impl Handler<TxResult> for Faucet {
    type Result = TxResponseResult;

    fn handle(&mut self, msg: TxResult, _: &mut Self::Context) -> Self::Result {
        info!("📝 Receive transaction result on faucet.");
        let message = FaucetTransactionMessage::build_message(
            msg.result,
            msg.subscribers,
            self.channel_id,
            self.explorer_url.clone(),
        );
        self.discord_client.do_send(SendMessage { message });
    }
}
