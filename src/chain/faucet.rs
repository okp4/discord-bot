//! Faucet module

use crate::chain::error::Error;
use bip39::Mnemonic;
use cosmrs::bip32::DerivationPath;
use cosmrs::crypto::secp256k1;
use cosmrs::AccountId;

const DERIVATION_PATH: &str = "m/44'/118'/0'/0/0";

/// Client to request cosmos blockchain for the faucet purpose
pub struct FaucetClient {
    /// Address of the faucet sender account.
    pub address: AccountId,

    /// Public/private key fo the faucet sender account.
    pub signing_key: secp256k1::SigningKey,
}

impl FaucetClient {

    /// Create a new faucet client based on the sender mnemonic account.
    pub fn new(mnemonic: &String) -> Result<Self, Error> {
        let mnemonic = Mnemonic::parse(mnemonic)?;

        let derivation_path: DerivationPath = DERIVATION_PATH.to_string().parse().unwrap();
        let sender_private_key =
            secp256k1::SigningKey::derive_from_path(mnemonic.to_seed(""), &derivation_path)?;
        let sender_public_key = sender_private_key.public_key();
        let address = sender_public_key.account_id("okp4")?;

        Ok(FaucetClient {
            address,
            signing_key: sender_private_key,
        })
    }
}
