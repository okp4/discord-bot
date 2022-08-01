//! Some convenient methods used for the discord client.

use serenity::model::application::interaction::Interaction;

/// Returns the name of the given interaction.
pub fn interation_name(interaction: &Interaction) -> String {
    String::from(match interaction {
        Interaction::Ping(_) => "ping",
        Interaction::ApplicationCommand(_) => "application-command",
        Interaction::MessageComponent(_) => "mesage-component",
        Interaction::Autocomplete(_) => "auto-complete",
        Interaction::ModalSubmit(_) => "modal-submit",
    })
}
