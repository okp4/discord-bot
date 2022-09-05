//! Holds CosmosClient library.

use crate::cosmos::client::error::Error;
use crate::discord_client::DiscordActor;
use actix::Addr;
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient as AuthClient;
use cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient;
use std::fmt::Debug;
use tonic::codegen::{Body, Bytes, StdError};
use tonic::transport::Channel;

pub mod account;
mod actor;
pub mod error;
mod handlers;
pub mod messages;

/// Hold all necessary client and service for cosmos chain
#[derive(Debug, Clone)]
pub struct Client<T> {
    channel: Box<T>,
    addr_discord_client: Addr<DiscordActor>,
}

impl<T> Client<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    /// returns the auth client module endpoint
    pub fn auth(self) -> AuthClient<T> {
        AuthClient::new(*self.channel)
    }

    /// return transaction service module endpoint
    pub fn tx(self) -> ServiceClient<T> {
        ServiceClient::new(*self.channel)
    }
}

impl Client<Channel> {
    /// Create a new client form a GRPC endpoint
    pub async fn new<D>(
        endpoint: D,
        addr_discord_client: Addr<DiscordActor>,
    ) -> Result<Client<Channel>, Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        tonic::transport::Endpoint::new(endpoint)
            .map_err(|err| Error::Connection(err.to_string()))?
            .connect()
            .await
            .map(|channel| Client {
                channel: Box::new(channel),
                addr_discord_client,
            })
            .map_err(|err| Error::Connection(err.to_string()))
    }
}
