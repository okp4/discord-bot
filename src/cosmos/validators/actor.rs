use std::time::Duration;

use actix::{Actor, AsyncContext, Context};
use tracing::info;

use crate::cosmos::validators::messages::get_state_message::GetStateMessage;
use crate::cosmos::validators::Validators;

impl Actor for Validators {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("📣 Validators status monitoring started...");
        ctx.run_interval(Duration::new(10, 0), |_act, ctx| {
            ctx.address().do_send(GetStateMessage {});
        });
    }
}
