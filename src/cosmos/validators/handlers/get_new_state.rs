use actix::{AsyncContext, ContextFutureSpawner, Handler, WrapFuture};
use cosmos_sdk_proto::cosmos::staking::v1beta1::QueryValidatorsResponse;
use tracing::{debug, info};

use crate::cosmos::client::messages::validators_status::GetValidatorsStatus;
use crate::cosmos::tx::error::Error;
use crate::cosmos::validators::messages::get_state_message::GetStateMessage;
use crate::cosmos::validators::messages::update_state_message::UpdateStateMessage;
use crate::cosmos::validators::Validators;
use crate::discord::discord_client::messages::send_msg::SendMessage;

impl Handler<GetStateMessage> for Validators {
    type Result = ();

    fn handle(&mut self, msg: GetStateMessage, ctx: &mut Self::Context) -> Self::Result {
        info!(
            "Get validators state, asynchronously send discord messages and update state {} {:?}",
            msg.bond_status.as_str_name(),
            msg.pagination_key
        );

        let grpc_client = self.grpc_client.clone();
        let discord_client = self.discord_client.clone();
        let channel_id = self.channel_id;
        let validators_current_state = self.validators_current.clone();
        let self_address = ctx.address();

        async move {
            let pagination_next_key: Vec<u8> = msg.pagination_key.unwrap_or(vec![]);

            let _ = grpc_client
                .send(GetValidatorsStatus {
                    status: msg.bond_status,
                    pagination_next_key,
                })
                .await
                .map_err(Error::from)
                .map(|response| {
                    response
                        .map_err(Error::from)
                        .map(|res: QueryValidatorsResponse| {
                            let validator_state = res.validators;

                            for message in Validators::compute_discord_message(
                                &validators_current_state,
                                &validator_state,
                            ) {
                                discord_client.do_send(SendMessage {
                                    description: message,
                                    title: "Validator state".to_string(),
                                    content: "".to_string(),
                                    channel_id,
                                });
                            }
                            self_address.do_send(UpdateStateMessage {
                                validators: validator_state,
                            });
                            match res.pagination {
                                None => debug!("pagination finished"),
                                Some(x) => {
                                    if x.next_key.len() > 0 {
                                        self_address.do_send(GetStateMessage {
                                            bond_status: msg.bond_status,
                                            pagination_key: Option::from(x.next_key),
                                        })
                                    }
                                }
                            }
                        })
                });
        }
        .into_actor(self)
        .wait(ctx);
    }
}
