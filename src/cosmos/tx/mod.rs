//! Holds Tx actors.

mod actor;
pub mod discord_message;
pub mod error;
pub mod handlers;
pub mod messages;

use crate::cosmos::client::account::Account;
use crate::cosmos::client::Client;
use crate::cosmos::tx::error::Error;
use actix::{Actor, Addr, Handler};
use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;
use cosmrs::tx::{Body, Fee, Msg, SignDoc, SignerInfo};
use serenity::model::user::User;
use std::time::Duration;
use tonic::transport::Channel;

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
    /// Common fees used for batch transaction
    pub fee: Fee,
    /// Duration between two transactions.
    pub batch_window: Duration,
    /// Contains the batch of transaction message to sent as prost::Any.
    msgs: Vec<T>,
    /// Contains the list of all user that request transaction.
    subscribers: Vec<User>,
    /// GRPC client to send transaction.
    grpc_client: Addr<Client<Channel>>,
}

impl<T> TxHandler<T>
where
    T: Msg + Unpin + 'static,
{
    /// Create a new TxHandler for a specific message type.
    pub fn new<F>(
        chain_id: String,
        sender: Account,
        fee: Fee,
        grpc_client: Addr<Client<Channel>>,
        f: F,
    ) -> TxHandler<T>
        where F: FnOnce(&mut TxHandler<T>) -> &mut TxHandler<T>, {
        let mut handler: TxHandler<T> = Self {
            chain_id,
            sender,
            memo: "".to_string(),
            fee,
            batch_window: Duration::new(8, 0),
            msgs: vec![],
            subscribers: vec![],
            grpc_client,
        };
        f(&mut handler);
        handler
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
