//! Holds Tx actors.

mod actor;
pub mod discord_message;
pub mod error;
pub mod handlers;
pub mod messages;

use std::collections::VecDeque;
use crate::cosmos::client::account::Account;
use crate::cosmos::client::Client;
use crate::cosmos::tx::error::Error;
use crate::cosmos::tx::messages::response::TxResult;
use actix::{Actor, Addr, Handler};
use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;
use cosmrs::tx::{Body, Fee, Msg, SignDoc, SignerInfo};
use serenity::model::user::User;
use std::time::Duration;
use tonic::transport::Channel;

/// Actor that will manage all transaction to the cosmos blockchain
/// Each transaction will be trigger each X seconds.
pub struct TxHandler<T, R>
where
    T: Msg + Unpin,
    R: Actor + Handler<TxResult>,
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
    msgs: VecDeque<(User, T)>,
    /// GRPC client to send transaction.
    grpc_client: Addr<Client<Channel>>,
    /// Hold address of actor that will receive message when a transaction has been broadcasted.
    response_handler: Option<Addr<R>>,
}

impl<T, R> TxHandler<T, R>
where
    T: Msg + Unpin + 'static,
    R: Actor + Handler<TxResult>,
{
    /// Create a new TxHandler for a specific message type.
    pub fn new<F>(
        chain_id: String,
        sender: Account,
        fee: Fee,
        grpc_client: Addr<Client<Channel>>,
        f: F,
    ) -> TxHandler<T, R>
    where
        F: FnOnce(&mut TxHandler<T, R>) -> &mut TxHandler<T, R>,
    {
        let mut handler: TxHandler<T, R> = Self {
            chain_id,
            sender,
            memo: "".to_string(),
            fee,
            batch_window: Duration::new(8, 0),
            msgs: VecDeque::new(),
            grpc_client,
            response_handler: None,
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
