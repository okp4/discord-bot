//! Faucet module

use crate::cosmos::grpc::error::Error;
use bip39::Mnemonic;
use cosmrs::bip32::DerivationPath;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::AccountId;

const DERIVATION_PATH: &str = "m/44'/118'/0'/0/0";

/// Client to request cosmos blockchain for the faucet purpose
pub struct Account {
    /// Address of the faucet sender account.
    pub address: AccountId,

    /// Account mnemonic.
    mnemonic: Mnemonic,
}

impl Account {
    /// Create a new faucet client based on the sender mnemonic account and address prefix.
    pub fn new(mnemonic: String, address_prefix: &str) -> Result<Self, Error> {
        let mnemonic = Mnemonic::parse(mnemonic)?;

        let private_key = Self::signing_key_from(&mnemonic)?;
        let address = private_key.public_key().account_id(address_prefix)?;

        Ok(Account { address, mnemonic })
    }

    /// Return the account private key
    pub fn signing_key(&self) -> Result<SigningKey, Error> {
        Self::signing_key_from(&self.mnemonic)
    }

    fn signing_key_from(mnemonic: &Mnemonic) -> Result<SigningKey, Error> {
        let derivation_path: DerivationPath = DERIVATION_PATH.to_string().parse().unwrap();
        SigningKey::derive_from_path(mnemonic.to_seed(""), &derivation_path).map_err(Error::from)
    }
}
