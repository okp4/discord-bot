use std::time::Duration;

use actix::{Actor, AsyncContext, Context};
use cosmos_sdk_proto::cosmos::staking::v1beta1::BondStatus;
use tracing::info;

use crate::cosmos::validators::messages::get_state_message::GetStateMessage;
use crate::cosmos::validators::Validators;

impl Actor for Validators {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("📣 Validators status monitoring started...");
        ctx.run_interval(Duration::new(120, 0), |_act, ctx| {
            for bond_status in [
                BondStatus::Unbonded,
                BondStatus::Bonded,
                BondStatus::Unbonding,
            ] {
                ctx.address().do_send(GetStateMessage {
                    bond_status,
                    pagination_key: None,
                });
            }
        });
    }
}
