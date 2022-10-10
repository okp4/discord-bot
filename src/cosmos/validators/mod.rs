//! Holds all the validator actor configuration

use actix::Addr;
use cosmos_sdk_proto::cosmos::staking::v1beta1::Validator;
use tonic::transport::Channel;
use tracing::error;

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

        for validator in current_validator_state {
            if !new_validator_state.contains(&validator) {
                messages.push(format!("💀 Lost validator: {}", validator.operator_address));
            }
        }

        for validator in new_validator_state {
            if !current_validator_state.contains(validator) {
                messages.push(format!("🎉 New validator: {}", validator.operator_address));
            } else {
                let old_state = current_validator_state.iter().find(|v|
                    (**v).eq(validator)
                );
                match old_state {
                    None => {
                        error!("Should have found {}, in current state", validator.operator_address);
                    }
                    Some(old_state) => {
                        if validator.jailed && !old_state.jailed {
                            messages.push(format!("🚓 Jailed validator {}", validator.operator_address));
                        } else if !validator.jailed && old_state.jailed {
                            messages.push(format!("🏁 {} is out of jail\nWelcome back!", validator.operator_address));
                        }
                    }
                }
            }
        }

        messages
    }
}
