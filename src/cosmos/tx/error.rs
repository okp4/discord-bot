//! All errors from the TxHandler actor.

use cosmrs::ErrorReport as CosmosErrorReport;
use thiserror::Error;

/// All possible error for a TxHandler actor.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    /// Failed to parse mnemonic.
    #[error("Chain id is incorrect")]
    IncorrectChain,

    /// Failed sign transaction.
    #[error("Failed sign transaction: {0}")]
    SignTx(String),

    /// Failed parse mnemonic to private key.
    #[error("Failed parse mnemonic to private key: {0}")]
    Mnemonic(String),
}

impl From<CosmosErrorReport> for Error {
    fn from(err: CosmosErrorReport) -> Self {
        Error::SignTx(err.to_string())
    }
}

impl From<bip39::Error> for Error {
    fn from(err: bip39::Error) -> Self {
        Error::Mnemonic(err.to_string())
    }
}

impl From<cosmrs::bip32::Error> for Error {
    fn from(err: cosmrs::bip32::Error) -> Self {
        Error::Mnemonic(err.to_string())
    }
}
