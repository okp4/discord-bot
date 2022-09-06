//! Holds Tx actors.

mod actor;
pub mod error;
pub mod handlers;
pub mod messages;

use crate::cosmos::client::account::Account;
use crate::cosmos::client::Client;
use crate::cosmos::tx::error::Error;
use crate::discord_client::DiscordActor;
use actix::Addr;
use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;
use cosmrs::tx::{Body, Fee, Msg, SignDoc, SignerInfo};
use cosmrs::Coin;
use serenity::model::user::User;
use std::time::Duration;
use tonic::transport::Channel;

/// Contains addresses of actors that will be used by the TxHandler
pub struct Actors {
    /// GRPC client to send transaction.
    pub grpc_client: Addr<Client<Channel>>,
    /// Address of the Discord client Actor
    pub discord_client: Addr<DiscordActor>,
}

/// Actor that will manage all transaction to the cosmos blockchain
/// Each transaction will be trigger each X seconds.
pub struct TxHandler<T>
where
    T: Msg + Unpin,
{
    /// Cosmos chain id.
    pub chain_id: String,
    /// Transaction sender .
    pub sender: Account,
    /// Common memo used for batch transaction
    pub memo: String,
    /// Common gas linit used for batch transaction
    pub gas_limit: u64,
    /// Common fee amount used for batch transaction
    pub fee_amount: Coin,
    /// Duration between two transactions.
    pub batch_window: Duration,
    /// Contains the batch of transaction message to sent as prost::Any.
    msgs: Vec<T>,
    /// Contains the list of all user that request transaction.
    subscribers: Vec<User>,
    /// GRPC client to send transaction.
    grpc_client: Addr<Client<Channel>>,
    /// Address of the Discord client Actor
    discord_client: Addr<DiscordActor>,
}

impl<T> TxHandler<T>
where
    T: Msg + Unpin + 'static,
{
    /// Create a new TxHandler for a specific message type.
    pub fn new(
        chain_id: String,
        sender: Account,
        memo: String,
        gas_limit: u64,
        fee_amount: Coin,
        batch_window: Duration,
        actors: Actors,
    ) -> TxHandler<T> {
        Self {
            chain_id,
            sender,
            memo,
            gas_limit,
            fee_amount,
            batch_window,
            msgs: vec![],
            subscribers: vec![],
            grpc_client: actors.grpc_client,
            discord_client: actors.discord_client,
        }
    }

    /// Sign a transaction messages.
    pub fn sign_tx(&self, body: &Body, account: BaseAccount, fee: Fee) -> Result<Vec<u8>, Error> {
        let public_key = self.sender.signing_key()?.public_key();
        let signer_info = SignerInfo::single_direct(Some(public_key), account.sequence);

        let auth_info = signer_info.auth_info(fee);

        let sign_doc = SignDoc::new(
            body,
            &auth_info,
            &self.chain_id.parse().map_err(|_| Error::IncorrectChain)?,
            account.account_number,
        )?;

        let tx_signed = sign_doc.sign(&self.sender.signing_key()?)?;

        tx_signed.to_bytes().map_err(Error::from)
    }
}
