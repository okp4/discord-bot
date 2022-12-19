//! Holds all the validators actor configuration

use actix::Addr;
use cosmos_sdk_proto::cosmos::staking::v1beta1::Validator;
use tonic::transport::Channel;

use crate::cosmos::client::Client;
use crate::discord::discord_client::DiscordActor;

mod actor;
mod discord_message;
mod discord_msg_util;
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
                .position(|v| v.operator_address.eq(&new_val.operator_address));
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
