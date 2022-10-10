use actix::{AsyncContext, ContextFutureSpawner, Handler, MailboxError, WrapFuture};
use tracing::info;

use crate::cosmos::client::messages::validators_status::{
    GetValidatorsStatus, GetValidatorsStatusResult,
};
use crate::cosmos::tx::error::Error;
use crate::cosmos::validators::Validators;
use crate::cosmos::validators::messages::get_state_message::GetStateMessage;
use crate::cosmos::validators::messages::update_state_message::UpdateStateMessage;
use crate::discord::discord_client::messages::send_msg::SendMessage;

impl Handler<GetStateMessage> for Validators {
    type Result = ();

    fn handle(&mut self, _msg: GetStateMessage, ctx: &mut Self::Context) -> Self::Result {
        info!("Validator get state, asynchronously send discord messages and update state");

        let grpc_client = self.grpc_client.clone();
        let discord_client = self.discord_client.clone();
        let channel_id = self.channel_id.clone();
        let validators_current_state = self.validators_current.clone();
        let self_address = ctx.address().clone();

        async move {
            let result: Result<GetValidatorsStatusResult, MailboxError> =
                grpc_client.send(GetValidatorsStatus {}).await;
            let _ = result.map_err(Error::from).and_then(|response| {
                Ok({
                    response
                        .map_err(Error::from)
                        .and_then(|res| Ok(res.validators))
                        .and_then(|validator_state| {
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
                                validators: validator_state.clone()
                            });
                            Ok({})
                        })
                })
            });
        }
            .into_actor(self).wait(ctx);
    }
}
