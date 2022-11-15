//! All errors from the TxHandler actor.

use actix::MailboxError;
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

    /// Failed to retrieve base account or account not found
    #[error("An error occur at the grpc client: {0}")]
    Client(crate::cosmos::client::error::Error),

    /// The transaction queue is full.
    #[error("The transaction queue is full")]
    QueueFull,

    /// A transaction is already registered for this user
    #[error("A transaction is already registered for this user")]
    DuplicateUser,

    /// Failed communicate to actix.
    #[error("An error occurs when send message to actix: {0}.")]
    Mailbox(String),

    /// Error occurs to lock mutex
    #[error("Error occurs to lock mutex")]
    Lock,
}

impl From<CosmosErrorReport> for Error {
    fn from(err: CosmosErrorReport) -> Self {
        Error::SignTx(err.to_string())
    }
}

impl From<crate::cosmos::client::error::Error> for Error {
    fn from(err: crate::cosmos::client::error::Error) -> Self {
        Error::Client(err)
    }
}

impl From<MailboxError> for Error {
    fn from(err: MailboxError) -> Self {
        Error::Mailbox(err.to_string())
    }
}
