use actix::Handler;
use tracing::debug;

use crate::cosmos::validators::messages::update_state_message::UpdateStateMessage;
use crate::cosmos::validators::Validators;

impl Handler<UpdateStateMessage> for Validators {
    type Result = ();
    fn handle(&mut self, msg: UpdateStateMessage, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Validators update state");

        msg.validators.iter().for_each(|new_val| {
            let val_pos = self
                .validators_current
                .iter()
                .position(|v| (*v).eq(new_val));
            match val_pos {
                None => {
                    self.validators_current.append(&mut msg.validators.clone());
                }
                Some(pos) => {
                    let _ = std::mem::replace(&mut self.validators_current[pos], new_val.clone());
                }
            }
            // info!("{:?}",self.validators_current);
        });
    }
}
