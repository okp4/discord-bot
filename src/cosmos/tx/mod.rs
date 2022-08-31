//! Holds Tx actors.

mod actor;
pub mod error;
pub mod handlers;
pub mod messages;

use crate::cosmos::client::account::Account;
use crate::cosmos::client::Client;
use crate::cosmos::tx::error::Error;
use actix::Addr;
use cosmrs::auth::BaseAccount;
use cosmrs::bank::MsgSend;
use cosmrs::tx::{Body, Fee, SignDoc, SignerInfo};
use tonic::transport::Channel;

/// Actor that will manage all transaction to the cosmos blockchain
/// Each transaction will be trigger each X seconds.
pub struct TxHandler {
    /// Cosmos chain id.
    pub chain_id: String,
    /// Transaction sender .
    pub sender: Account,
    /// Common memo used for batch transaction
    pub memo: String,
    /// Common gas linit used for batch transaction
    pub gas_limit: u64,
    /// Contains the batch of transaction message to sent.
    pub msgs: Vec<MsgSend>,
    /// GRPC client to send transaction.
    pub grpc_client: Addr<Client<Channel>>,
}

impl TxHandler {
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
