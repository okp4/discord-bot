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
