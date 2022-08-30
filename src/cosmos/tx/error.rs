use thiserror::Error;
use cosmrs::ErrorReport as CosmosErrorReport;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    /// Failed to parse mnemonic.
    #[error("Chain id is incorrect")]
    IncorrectChain,

    /// Failed sign transaction.
    #[error("Failed sign transaction: {0}")]
    SignTx(String),
}

impl From<CosmosErrorReport> for Error {
    fn from(err: CosmosErrorReport) -> Self {
        Error::SignTx(err.to_string())
    }
}
