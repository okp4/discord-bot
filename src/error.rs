//! Error types

use abscissa_core::error::{BoxError, Context};
use serenity::Error as SerenityError;
use std::{
    fmt::{self, Display},
    io,
    ops::Deref,
};
use thiserror::Error;

/// Kinds of errors
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ErrorKind {
    /// Error in configuration file
    #[error("config error")]
    Config,

    /// Input/output error
    #[error("I/O error")]
    Io,

    /// Input/output error
    #[error("Client error")]
    Client(String),

    /// Errors from Serenity
    #[error("Serenity Error {0}")]
    SerenityError(String),

    /// Cosmos Error
    #[error("Cosmos error")]
    CosmosError(String),

    /// Cosmos client Error
    #[error("Cosmos client error")]
    CosmosClientError(String),
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

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        ErrorKind::Io.context(err).into()
    }
}

impl From<SerenityError> for Error {
    fn from(err: SerenityError) -> Self {
        ErrorKind::SerenityError(err.to_string())
            .context(err)
            .into()
    }
}

impl From<cosmrs::Error> for Error {
    fn from(err: cosmrs::Error) -> Self {
        ErrorKind::CosmosError(err.to_string()).context(err).into()
    }
}

impl From<crate::cosmos::client::error::Error> for Error {
    fn from(err: crate::cosmos::client::error::Error) -> Self {
        ErrorKind::CosmosClientError(err.to_string())
            .context(err)
            .into()
    }
}
