//! Define trait for all transaction result message

use crate::cosmos::tx::error::Error as TxError;
use cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use serenity::model::user::User;

/// Trait for all transaction discord message
pub trait TransactionDiscordMessage {
    /// Build the message with the transaction response or failure error
    fn build_message(
        tx_response: Result<TxResponse, TxError>,
        subscribers: Vec<User>,
        channel_id: u64,
    ) -> Self;
}
