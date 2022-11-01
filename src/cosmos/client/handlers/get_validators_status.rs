use actix::prelude::*;
use actix::Handler;
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::staking::v1beta1::QueryValidatorsRequest;
use tonic::transport::Channel;
use tracing::{debug, error};

use crate::cosmos::client::messages::validators_status::{
    GetValidatorsStatus, GetValidatorsStatusResult,
};
use crate::cosmos::client::Client;

impl Handler<GetValidatorsStatus> for Client<Channel> {
    type Result = ResponseFuture<GetValidatorsStatusResult>;

    fn handle(&mut self, msg: GetValidatorsStatus, _ctx: &mut Self::Context) -> Self::Result {
        let mut validator_client = self.clone().validator();
        Box::pin(async move {
            debug!(
                "handle get validators status request {} {:?}",
                msg.status.as_str_name(),
                msg.pagination_next_key
            );

            let page_request = PageRequest {
                key: msg.pagination_next_key,
                offset: 0,
                limit: 0,
                count_total: true,
                reverse: false,
            };

            let response = validator_client
                .validators(tonic::Request::new(QueryValidatorsRequest {
                    status: msg.status.as_str_name().parse().unwrap(),
                    pagination: Option::from(page_request),
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
