//! Holds all the validators actor configuration

use actix::Addr;
use cosmos_sdk_proto::cosmos::staking::v1beta1::Validator;
use tonic::transport::Channel;
use tracing::debug;

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
    grpc_client: Option<Addr<Client<Channel>>>,
    /// Address of the Discord client Actor
    discord_client: Option<Addr<DiscordActor>>,
    /// the actual state of all the validators
    validators_current: Vec<Validator>,
}

enum Message {
    Jailed,
    Unjailed,
    Inactive,
    Active,
}

fn msg_to_str(msg: Message, subject: String) -> String {
    match msg {
        Message::Jailed => format!("🚓 Jailed validator : `{}`", subject),
        Message::Unjailed => format!("🏁 `{}` is out of jail\nWelcome back!", subject),
        Message::Inactive => format!("😵 `{}` is inactive", subject),
        Message::Active => format!("🥳 {} is active", subject),
    }
}

impl Validators {
    /// Create a new validators actor client
    pub fn new(
        channel_id: u64,
        grpc_client: Option<Addr<Client<Channel>>>,
        discord_client: Option<Addr<DiscordActor>>,
    ) -> Self {
        Validators {
            channel_id,
            grpc_client,
            discord_client,
            validators_current: vec![],
        }
    }
    fn update_state(&mut self, validators: Vec<Validator>) {
        validators.iter().for_each(|new_val| {
            let val_pos = self
                .validators_current
                .iter()
                .position(|v| (*v).operator_address.eq(&new_val.operator_address));
            match val_pos {
                None => {
                    self.validators_current.append(&mut validators.clone());
                }
                Some(pos) => {
                    self.validators_current[pos] = new_val.clone();
                }
            }
        });
    }

    fn compute_discord_message(
        current_validator_state: &[Validator],
        new_validator_state: &[Validator],
    ) -> Vec<String> {
        let mut messages: Vec<String> = vec![];

        for validator in new_validator_state {
            let name_to_display = validator
                .description
                .as_ref()
                .map_or_else(|| validator.operator_address.clone(), |d| d.moniker.clone());

            let old_state = current_validator_state.iter().find(|v| {
                ((**v).operator_address.clone().as_str())
                    .eq(validator.operator_address.clone().as_str())
            });
            match old_state {
                None => {
                    debug!("New validator : {}", name_to_display.clone())
                }
                Some(old_state) => {
                    if validator.jailed && !old_state.jailed {
                        messages.push(msg_to_str(Message::Jailed, name_to_display.clone()));
                    } else if !validator.jailed && old_state.jailed {
                        messages.push(msg_to_str(Message::Unjailed, name_to_display.clone()));
                    }

                    if validator.status != old_state.status {
                        if validator.status == 3 {
                            messages.push(msg_to_str(Message::Active, name_to_display.clone()));
                        } else {
                            messages.push(msg_to_str(Message::Inactive, name_to_display.clone()));
                        }
                    }
                }
            }
        }
        messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_state_all_empty() {
        let mut validators = Validators::new(123, None, None);
        validators.update_state(vec![]);
        assert_eq!(validators.validators_current, vec![]);
    }

    #[test]
    fn test_update_state_empty() {
        let mut validators = Validators::new(123, None, None);
        let val1 = Validator {
            status: 1,
            tokens: "".to_string(),
            delegator_shares: "".to_string(),
            description: None,
            unbonding_height: 0,
            unbonding_time: None,
            commission: None,
            operator_address: "123456".to_string(),

            consensus_pubkey: None,
            jailed: false,
            min_self_delegation: "".to_string(),
        };
        validators.update_state(vec![val1]);
        assert_eq!(validators.validators_current.len(), 1);
    }

    #[test]
    fn test_update_state_actual_update() {
        let mut validators = Validators::new(123, None, None);
        let val1 = Validator {
            status: 1,
            tokens: "".to_string(),
            delegator_shares: "".to_string(),
            description: None,
            unbonding_height: 0,
            unbonding_time: None,
            commission: None,
            operator_address: "123456".to_string(),

            consensus_pubkey: None,
            jailed: false,
            min_self_delegation: "".to_string(),
        };
        validators.update_state(vec![val1]);
        assert_eq!(validators.validators_current[0].status, 1);

        let val2 = Validator {
            status: 2,
            tokens: "".to_string(),
            delegator_shares: "".to_string(),
            description: None,
            unbonding_height: 0,
            unbonding_time: None,
            commission: None,
            operator_address: "123456".to_string(),

            consensus_pubkey: None,
            jailed: false,
            min_self_delegation: "".to_string(),
        };
        validators.update_state(vec![val2]);
        assert_eq!(validators.validators_current.len(), 1);
        assert_eq!(validators.validators_current[0].status, 2);
    }
}
