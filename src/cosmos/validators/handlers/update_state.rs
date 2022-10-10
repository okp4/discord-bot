use actix::Handler;
use tracing::debug;

use crate::cosmos::validators::messages::update_state_message::UpdateStateMessage;
use crate::cosmos::validators::Validators;

impl Handler<UpdateStateMessage> for Validators {
    type Result = ();
    fn handle(&mut self, msg: UpdateStateMessage, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Validators update state");
        self.validators_current.clear();
        self.validators_current.append(&mut msg.validators.clone())
    }
}
