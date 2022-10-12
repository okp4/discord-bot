use actix::prelude::*;
use actix::Handler;
use cosmos_sdk_proto::cosmos::staking::v1beta1::QueryValidatorsRequest;
use tonic::transport::Channel;
use tracing::{error, info};

use crate::cosmos::client::messages::validators_status::{
    GetValidatorsStatus, GetValidatorsStatusResult,
};
use crate::cosmos::client::Client;

impl Handler<GetValidatorsStatus> for Client<Channel> {
    type Result = ResponseFuture<GetValidatorsStatusResult>;

    fn handle(&mut self, msg: GetValidatorsStatus, _ctx: &mut Self::Context) -> Self::Result {
        let mut validator_client = self.clone().validator();
        Box::pin(async move {
            info!("handle get validators status request {}", msg.status.as_str_name());

            let response = validator_client
                .validators(tonic::Request::new(QueryValidatorsRequest {
                    status: msg.status.as_str_name().parse().unwrap(),
                    pagination: None,
                }))
                .await
                .map_err(|err| {
                    error!(
                        "☠ Failed to get validator state: {} {}",
                        err.code(),
                        err.message()
                    );
                    err
                })?;

            Ok(response.get_ref().clone())
        })
    }
}
