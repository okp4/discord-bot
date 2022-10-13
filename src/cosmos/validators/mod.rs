//! Holds all the validator actor configuration

use actix::Addr;
use cosmos_sdk_proto::cosmos::staking::v1beta1::Validator;
use tonic::transport::Channel;

use crate::cosmos::client::Client;
use crate::discord::discord_client::DiscordActor;

mod actor;
mod handlers;
mod messages;

/// A Validators actor that will send discord message depending on
/// validators changing states
pub struct Validators {
    /// Set the discord channel validator status should be sent.
    channel_id: u64,
    /// GRPC client to send transaction.
    grpc_client: Addr<Client<Channel>>,
    /// Address of the Discord client Actor
    discord_client: Addr<DiscordActor>,
    /// the actual state of all the validators
    validators_current: Vec<Validator>,
}

impl Validators {
    /// Create a new validators actor client
    pub fn new(
        channel_id: u64,
        grpc_client: Addr<Client<Channel>>,
        discord_client: Addr<DiscordActor>,
    ) -> Self {
        Validators {
            channel_id,
            grpc_client,
            discord_client,
            validators_current: vec![],
        }
    }

    fn compute_discord_message(
        current_validator_state: &Vec<Validator>,
        new_validator_state: &Vec<Validator>,
    ) -> Vec<String> {
        let mut messages: Vec<String> = vec![];

        for validator in new_validator_state {
            let name_to_display = validator.description.as_ref().map_or_else(|| validator.operator_address.clone(),
                                                                             |d| d.moniker.clone());

            let old_state = current_validator_state.iter().find(|v|
                (**v).eq(validator)
            );
            match old_state {
                None => {
                    messages.push(format!("🎉 New validator: {}", name_to_display));
                }
                Some(old_state) => {
                    if validator.jailed && !old_state.jailed {
                        messages.push(format!("🚓 Jailed validator {}", name_to_display));
                    } else if !validator.jailed && old_state.jailed {
                        messages.push(format!("🏁 {} is out of jail\nWelcome back!", name_to_display));
                    }

                    if validator.status != old_state.status {
                        messages.push(format!("⚠️ {} status changed : {} ➡️ {}", name_to_display, get_status_txt(old_state.status), get_status_txt(validator.status)));
                    }
                }
            }
        }

        messages
    }
}

fn get_status_txt(value: i32) -> String {
    match value {
        1 => "unbonded".to_string(),
        2 => "unbonding".to_string(),
        3 => "bonded".to_string(),
        _ => "Unspecified".to_string(),
    }
}
