//! Holds Tx actors.

mod actor;
pub mod discord_message;
pub mod error;
pub mod handlers;
pub mod messages;

use std::marker::PhantomData;
use crate::cosmos::client::account::Account;
use crate::cosmos::client::Client;
use crate::cosmos::tx::discord_message::TransactionDiscordMessage;
use crate::cosmos::tx::error::Error;
use crate::discord::discord_client::message::DiscordMessage;
use crate::discord::discord_client::DiscordActor;
use actix::Addr;
use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;
use cosmrs::tx::{Body, Fee, Msg, SignDoc, SignerInfo};
use serenity::model::user::User;
use std::time::Duration;
use tonic::transport::Channel;

/// Contains addresses of actors that will be used by the TxHandler
pub struct Actors
{
    /// GRPC client to send transaction.
    pub grpc_client: Addr<Client<Channel>>,
    /// Address of the Discord client Actor
    pub discord_client: Addr<DiscordActor>,
}

/// Actor that will manage all transaction to the cosmos blockchain
/// Each transaction will be trigger each X seconds.
pub struct TxHandler<T, M>
where
    T: Msg + Unpin,
    M: TransactionDiscordMessage + DiscordMessage + Unpin + 'static,
{
    /// Cosmos chain id.
    pub chain_id: String,
    /// Transaction sender .
    pub sender: Account,
    /// Common memo used for batch transaction
    pub memo: String,
    /// Common fees used for batch transaction
    pub fee: Fee,
    /// Duration between two transactions.
    pub batch_window: Duration,
    /// Set the discord channel where to send transaction result.
    pub channel_id: Option<u64>,
    /// Contains the batch of transaction message to sent as prost::Any.
    msgs: Vec<T>,
    /// Contains the list of all user that request transaction.
    subscribers: Vec<User>,
    /// GRPC client to send transaction.
    grpc_client: Addr<Client<Channel>>,
    /// Address of the Discord client Actor
    discord_client: Addr<DiscordActor>,
    /// To tell compiler that the message type is of type M and to avoid unused generic parameter compilation error.
    /// This is mandatory if we would like to instantiate the message with it's static method `::build_message`.
    /// See [E0392](https://doc.rust-lang.org/error-index.html#E0392) for more detail of usage of PhantomData.
    phantom: PhantomData<M>
}

impl<T, M> TxHandler<T, M>
where
    T: Msg + Unpin + 'static,
    M: TransactionDiscordMessage + Unpin + DiscordMessage,
{
    /// Create a new TxHandler for a specific message type.
    pub fn new(
        chain_id: String,
        sender: Account,
        memo: String,
        fee: Fee,
        batch_window: Duration,
        channel_id: Option<u64>,
        actors: Actors,
    ) -> TxHandler<T, M> {
        Self {
            chain_id,
            sender,
            memo,
            fee,
            batch_window,
            channel_id,
            msgs: vec![],
            subscribers: vec![],
            grpc_client: actors.grpc_client,
            discord_client: actors.discord_client,
            phantom: PhantomData
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
