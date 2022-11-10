//! Implementation of a faucet discord message result for after transaction

use crate::cosmos::tx::discord_message::TransactionDiscordMessage;
use crate::cosmos::tx::error::Error as TxError;
use crate::discord::discord_client::message::DiscordMessage;
use cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::model::user::User;
use serenity::prelude::Mentionable;
use serenity::Error;
use tracing::{error, info};

/// The faucet response message when transaction successful
#[derive(Clone, Debug)]
pub struct FaucetTransactionMessage {
    /// Title of the embedded message - optional
    pub title: String,
    /// Description of the embedded message - optional
    pub description: String,
    /// Content of the message body - optional
    pub content: String,
    /// Channel to send into
    pub channel_id: u64,
}

impl TransactionDiscordMessage for FaucetTransactionMessage {
    fn build_message(
        tx_response: Result<TxResponse, TxError>,
        subscribers: Vec<User>,
        channel_id: u64,
    ) -> Self {
        match tx_response {
            Ok(tx_response) => {
                info!(
                    "✅ Transaction successfully broadcasted : {}",
                    tx_response.txhash
                );
                Self {
                    title: String::from("🚀 Transaction broadcasted!"),
                    description: format!(
                        "\t- 🤝 Transaction hash: {}
                            \t- ⚙️ Result code : {}
                            \t- ⛽️ Gas used: {}",
                        tx_response.txhash, tx_response.code, tx_response.gas_used
                    ),
                    content: {
                        let mut str = String::new();
                        for sub in subscribers {
                            str.push_str(
                                &format_args!("{member} ", member = &sub.mention()).to_string(),
                            );
                        }
                        str
                    },
                    channel_id,
                }
            }
            Err(why) => {
                error!("❌ Failed broadcast transaction {}", why);
                Self {
                    title: String::from("🤷 So sorry, something went wrong"),
                    description: String::from(
                        "You're request was not processed.\nThe transaction was not broadcasted.",
                    ),
                    content: {
                        let mut str = String::new();
                        for sub in subscribers {
                            str.push_str(
                                &format_args!("{member} ", member = &sub.mention()).to_string(),
                            );
                        }
                        str
                    },
                    channel_id,
                }
            }
        }
    }
}

#[async_trait]
impl DiscordMessage for FaucetTransactionMessage {
    async fn send_message(self, http: &Http) -> Result<Message, Error> {
        ChannelId(self.channel_id)
            .send_message(&http, |m| {
                m.content(self.content);
                if !self.title.is_empty() || !self.description.is_empty() {
                    m.embed(|e| {
                        if !self.title.is_empty() {
                            e.title(self.title);
                        }
                        if !self.description.is_empty() {
                            e.description(self.description);
                        }
                        e
                    });
                }
                m
            })
            .await
    }
}
