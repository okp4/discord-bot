use actix::Handler;
use cosmos_sdk_proto::cosmos::auth::v1beta1::QueryAccountRequest;
use cosmrs::Error as CosmosError;
use tonic::transport::Channel;

use crate::cosmos::client::{
    error::Error,
    messages::get_account::{GetAccount, GetAccountResult},
    Client,
};

impl Handler<GetAccount> for Client<Channel> {
    type Result = GetAccountResult;

    fn handle(&mut self, msg: GetAccount, _ctx: &mut Self::Context) -> Self::Result {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(async move {
                let response = self
                    .clone()
                    .auth()
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

                Ok(unpack_from_any(account_response)
                    .ok_or(CosmosError::AccountId { id: msg.addr })?)
            })
    }
}

fn unpack_from_any<M>(msg: &prost_types::Any) -> Option<M>
where
    M: prost::Message + Default,
{
    M::decode(&msg.value[..]).ok()
}
