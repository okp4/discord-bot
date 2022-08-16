//! Error types

use crate::chain::error::Error as ChainError;
use abscissa_core::error::{BoxError, Context};
use cosmrs::Error as CosmosError;
use serenity::Error as SerenityError;
use std::{
    fmt::{self, Display},
    ops::Deref,
};
use thiserror::Error;
use tonic::Status;

/// Kinds of errors
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ErrorKind {
    /// Unknown command execution error
    #[error("Unknown error")]
    UnknownCommand(String),

    /// Missing arg for command
    #[error("Missing {0} arg")]
    MissingArg(String),

    /// Arg is incorrect
    #[error("{0} arg is incorrect {1}")]
    IncorrectArg(String, String),

    /// Errors from Serenity
    #[error("Serenity Error {0}")]
    Serenity(String),

    /// Errors from the grpc client
    #[error("GRPC client error : {0}")]
    Chain(ChainError),

    /// Request error
    #[error("Request error : {0}")]
    Status(String),
}

impl ErrorKind {
    /// Create an error context from this error
    pub fn context(self, source: impl Into<BoxError>) -> Context<ErrorKind> {
        Context::new(self, Some(source.into()))
    }
}

/// Error type
#[derive(Debug)]
pub struct Error(Box<Context<ErrorKind>>);

impl Deref for Error {
    type Target = Context<ErrorKind>;

    fn deref(&self) -> &Context<ErrorKind> {
        &self.0
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Context::new(kind, None).into()
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(context: Context<ErrorKind>) -> Self {
        Error(Box::new(context))
    }
}

impl From<SerenityError> for Error {
    fn from(err: SerenityError) -> Self {
        ErrorKind::Serenity(err.to_string()).context(err).into()
    }
}

impl From<ChainError> for Error {
    fn from(err: ChainError) -> Self {
        Error::from(ErrorKind::Chain(err))
    }
}

impl From<CosmosError> for Error {
    fn from(err: CosmosError) -> Self {
        Error::from(ChainError::from(err))
    }
}

impl From<Status> for Error {
    fn from(err: Status) -> Self {
        Error::from(ErrorKind::Status(err.to_string()))
    }
}
