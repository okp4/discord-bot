use cosmos_sdk_proto::cosmos::staking::v1beta1::Validator;
use tracing::debug;

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
        Message::Active => format!("🥳 `{}` is active", subject),
    }
}

pub fn compute_discord_message(
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
            (v.operator_address.clone().as_str()).eq(validator.operator_address.clone().as_str())
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmos_sdk_proto::cosmos::staking::v1beta1::Description;

    fn get_description() -> Option<Description> {
        Option::from(Description {
            moniker: "lemmings".to_string(),
            identity: "".to_string(),
            website: "".to_string(),
            security_contact: "".to_string(),
            details: "".to_string(),
        })
    }

    fn get_active() -> Validator {
        Validator {
            status: 3,
            tokens: "".to_string(),
            delegator_shares: "".to_string(),
            description: get_description(),
            unbonding_height: 0,
            unbonding_time: None,
            commission: None,
            operator_address: "123456".to_string(),

            consensus_pubkey: None,
            jailed: false,
            min_self_delegation: "".to_string(),
        }
    }

    fn get_inactive() -> Validator {
        Validator {
            status: 1,
            tokens: "".to_string(),
            delegator_shares: "".to_string(),
            description: get_description(),
            unbonding_height: 0,
            unbonding_time: None,
            commission: None,
            operator_address: "123456".to_string(),

            consensus_pubkey: None,
            jailed: false,
            min_self_delegation: "".to_string(),
        }
    }

    #[test]
    fn test_active() {
        let val1 = get_inactive();
        let val2 = get_active();

        let old_validators = [val1];
        let new_validators = [val2];

        let messages = compute_discord_message(old_validators.as_ref(), new_validators.as_ref());

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "🥳 `lemmings` is active");
    }

    #[test]
    fn test_inactive() {
        let val1 = get_active();
        let val2 = get_inactive();

        let old_validators = [val1];
        let new_validators = [val2];

        let messages = compute_discord_message(old_validators.as_ref(), new_validators.as_ref());

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "😵 `lemmings` is inactive");
    }

    #[test]
    fn test_nothing_hanged() {
        let val1 = get_active();
        let val2 = get_active();

        let old_validators = [val1];
        let new_validators = [val2];

        let messages = compute_discord_message(old_validators.as_ref(), new_validators.as_ref());

        assert_eq!(messages.len(), 0);
    }

    #[test]
    fn test_jailed() {
        let mut val1 = get_active();
        val1.jailed = false;
        let mut val2 = get_active();
        val2.jailed = true;

        let old_validators = [val1];
        let new_validators = [val2];

        let messages = compute_discord_message(old_validators.as_ref(), new_validators.as_ref());

        assert_eq!(messages[0], "🚓 Jailed validator : `lemmings`");
    }

    #[test]
    fn test_unjailed() {
        let mut val1 = get_active();
        val1.jailed = true;
        let mut val2 = get_active();
        val2.jailed = false;

        let old_validators = [val1];
        let new_validators = [val2];

        let messages = compute_discord_message(old_validators.as_ref(), new_validators.as_ref());

        assert_eq!(messages[0], "🏁 `lemmings` is out of jail\nWelcome back!");
    }
}
