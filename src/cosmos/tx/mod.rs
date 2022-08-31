//! Holds Tx actors.

mod actor;
pub mod error;
pub mod handlers;
pub mod messages;

use actix::Addr;
use crate::cosmos::tx::error::Error;
use bip39::Mnemonic;
use cosmrs::auth::BaseAccount;
use cosmrs::bank::MsgSend;
use cosmrs::bip32::DerivationPath;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::{Body, Fee, SignDoc, SignerInfo};
use tonic::transport::Channel;
use crate::cosmos::client::Client;

const DERIVATION_PATH: &str = "m/44'/118'/0'/0/0";

/// Actor that will manage all transaction to the cosmos blockchain
/// Each transaction will be trigger each X seconds.
pub struct TxHandler {
    /// Cosmos chain id.
    pub chain_id: String,
    /// Transaction sender mnemonic.
    pub mnemonic: String,
    /// Contains the batch of transaction message to sent.
    pub msgs: Vec<MsgSend>,
    /// GRPC client to send transaction.
    pub grpc_client: Addr<Client<Channel>>
}

impl TxHandler {
    /// Sign a transaction messages.
    pub fn sign_tx(
        &self,
        body: &Body,
        account: BaseAccount,
        signing_key: SigningKey,
        fee: Fee,
    ) -> Result<Vec<u8>, Error> {
        let public_key = signing_key.public_key();
        let signer_info = SignerInfo::single_direct(Some(public_key), account.sequence);

        let auth_info = signer_info.auth_info(fee);

        let sign_doc = SignDoc::new(
            body,
            &auth_info,
            &self.chain_id.parse().map_err(|_| Error::IncorrectChain)?,
            account.account_number,
        )?;

        let tx_signed = sign_doc.sign(&signing_key)?;

        tx_signed.to_bytes().map_err(Error::from)
    }

    /// Return the private key parsed from the mnemonic.
    pub(crate) fn signing_key(&self) -> Result<SigningKey, Error> {
        let mnemonic = Mnemonic::parse(&self.mnemonic)?;
        let derivation_path: DerivationPath = DERIVATION_PATH.to_string().parse().unwrap();
        SigningKey::derive_from_path(mnemonic.to_seed(""), &derivation_path).map_err(Error::from)
    }
}
