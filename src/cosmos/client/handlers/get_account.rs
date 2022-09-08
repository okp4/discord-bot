use actix::prelude::*;
use actix::Handler;
use cosmos_sdk_proto::cosmos::auth::v1beta1::QueryAccountRequest;
use cosmrs::Error as CosmosError;
use tonic::transport::Channel;

use crate::cosmos::client::error::Error;
use crate::cosmos::client::{
    messages::get_account::{GetAccount, GetAccountResult},
    Client,
};

impl Handler<GetAccount> for Client<Channel> {
    type Result = ResponseFuture<GetAccountResult>;

    fn handle(&mut self, msg: GetAccount, _ctx: &mut Self::Context) -> Self::Result {
        let mut auth_client = self.clone().auth();
        Box::pin(async move {
            let response = auth_client
                .account(tonic::Request::new(QueryAccountRequest {
                    address: msg.addr.to_string(),
                }))
                .await
                .map_err(Error::from)?;

            let account_response = response
                .get_ref()
                .account
                .as_ref()
                .ok_or(CosmosError::AccountId {
                    id: msg.addr.to_string(),
                })
                .map_err(Error::from)?;

            Ok(unpack_from_any(account_response).ok_or(CosmosError::AccountId { id: msg.addr })?)
        })
    }
}

fn unpack_from_any<M>(msg: &prost_types::Any) -> Option<M>
where
    M: prost::Message + Default,
{
    M::decode(&msg.value[..]).ok()
}
