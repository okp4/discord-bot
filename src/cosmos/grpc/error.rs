//! Error for grpc clients

use crate::cosmos::grpc::error::Error::{InvalidMnemonic, MnemonicParseFailed};
use bip39::Error as Bip39Error;
use cosmrs::bip32::Error as Bip32Error;
use cosmrs::proto::prost::EncodeError;
use cosmrs::tendermint::Error as TendermintError;
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

    /// Invalid grpc uri
    #[error("Failed connect to grpc endpoint: {0}")]
    Connection(String),

    /// Cosmos error.
    #[error("Cosmos error : {0}")]
    Cosmos(CosmosError),

    /// Cosmos error.
    #[error("Tendermint error : {0}")]
    Tendermint(String),

    /// Prost encode error.
    #[error("Protobuf encoding error: {0}")]
    Encode(EncodeError),

    /// Client not initialized
    #[error("GRPC client not initialized")]
    NotInitialized,
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

impl From<EncodeError> for Error {
    fn from(err: EncodeError) -> Self {
        Error::Encode(err)
    }
}

impl From<TendermintError> for Error {
    fn from(err: TendermintError) -> Self {
        Error::Tendermint(err.to_string())
    }
}
