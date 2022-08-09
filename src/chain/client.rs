//! GRPC clients

use std::fmt::Debug;
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient as AuthClient;
use tonic::codegen::{Body, Bytes, StdError};
use tonic::transport::Channel;
use crate::chain::error::Error;

/// Hold all necessary client and service for cosmos chain
#[derive(Debug, Clone)]
pub struct Client<T> {
    channel: T,
}

impl<T> Client<T>
    where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send, {

    // pub async fn new<T>(inner: T) -> Client<T> {
    //     Self { channel: inner }
    // }

    /// returns the auth client module endpoint
    pub fn auth(self) -> AuthClient<T> {
        AuthClient::new(self.channel)
    }
}

impl Client<Channel> {

    /// Create a new client form a GRPC endpoint
    pub async fn new<D>(endpoint: D) -> Result<Self, Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
    {
        tonic::transport::Endpoint::new(endpoint)
            .map_err(|err| Error::Connection(err.to_string()))?
            .connect().await
            .map(|channel| Client { channel })
            .map_err(|err| Error::Connection(err.to_string()))
    }
}
