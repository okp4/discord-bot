//! Error for grpc clients

use crate::chain::error::Error::{InvalidMnemonic, MnemonicParseFailed};
use bip39::Error as Bip39Error;
use cosmrs::bip32::Error as Bip32Error;
use cosmrs::Error as CosmosError;
use cosmrs::Error::Crypto;
use cosmrs::ErrorReport as CosmosErrorReport;
use thiserror::Error;

/// Chain errors
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    /// Failed to parse mnemonic.
    #[error("Failed parse mnemonic : {0}")]
    MnemonicParseFailed(Bip39Error),

    /// Mnemonic seems to be incorrect.
    #[error("Invalid mnemonic : {0}")]
    InvalidMnemonic(Bip32Error),

    /// Cosmos error.
    #[error("Cosmos error : {0}")]
    Cosmos(CosmosError),
}

impl From<Bip39Error> for Error {
    fn from(err: Bip39Error) -> Self {
        MnemonicParseFailed(err)
    }
}

impl From<Bip32Error> for Error {
    fn from(err: Bip32Error) -> Self {
        InvalidMnemonic(err)
    }
}

impl From<CosmosError> for Error {
    fn from(err: CosmosError) -> Self {
        Error::Cosmos(err)
    }
}

impl From<CosmosErrorReport> for Error {
    fn from(_: CosmosErrorReport) -> Self {
        Error::Cosmos(Crypto)
    }
}
