use colored::Colorize;
use rogger::*;
use serenity::{
    builder::{CreateButton, CreateEmbed},
    model::{
        channel::AttachmentType,
        prelude::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::Context,
};
use std::collections::HashMap;

/// Some interactions.
pub enum Interactions<'a> {
    /// Do some interaction with the response.
    Some(Vec<InteractMode<'a>>),
    /// Edit the message.
    ///
    /// # Note
    ///
    /// Cannot attach files at edit.
    Edit(Vec<InteractMode<'a>>),
    /// Send "Not implemented yet :<".
    Dev,
    /// Do nothing.
    None,
}

impl Interactions<'_> {
    /// Whether the interaction is `Edit(Vec<InteractMode<'a>>)`.
    pub fn is_edit(&self) -> bool {
        matches!(self, Interactions::Edit(_))
    }
}

/// The interaction mode.
pub enum InteractMode<'a> {
    /// Add a message.
    Message(String),
    /// Add a attachment.
    Attach(AttachmentType<'a>),
    /// Add a embed.
    Embed(CreateEmbed),
    /// Add a button.
    Button(CreateButton),
}

pub async fn handle_err(
    is_edit: bool,
    err: serenity::Error,
    interact: &ApplicationCommandInteraction,
    ctx: Context,
    dict: HashMap<String, (String, String)>,
) {
    let err_msg = err.to_string();

    if let serenity::Error::Http(http_err) = err {
        if let serenity::http::error::Error::UnsuccessfulRequest(res) = *http_err {
            let mut is_btml_error = false;
            for e in res.error.errors {
                if e.code == "BASE_TYPE_MAX_LENGTH" {
                    is_btml_error = true;
                }
            }
            if is_btml_error {
                let msg = dict.get("msg").unwrap();
                let is_locale_ja = interact.locale == "ja";
                if is_edit {
                    interact
                        .edit_original_interaction_response(&ctx.http, |res| {
                            res.content(if is_locale_ja { &msg.1 } else { &msg.0 })
                        })
                        .await
                        .expect("Failed to edit a `BASE_TYPE_MAX_LENGTH` message");
                } else {
                    interact
                        .create_interaction_response(&ctx.http, |res| {
                            res.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|m| {
                                    m.content(if is_locale_ja { &msg.1 } else { &msg.0 })
                                        .ephemeral(true)
                                })
                        })
                        .await
                        .expect("Failed to send a `BASE_TYPE_MAX_LENGTH` message");
                }
                warn!(
                    "Could not respond to `{}` command due to character limit. Args: {:?}",
                    interact.data.name.as_str(),
                    interact.data.options
                );
            } else {
                error!("Cannot respond to interaction: {}", err_msg);
            }
        } else {
            error!("Cannot respond to interaction: {}", err_msg);
        }
    } else {
        error!("Cannot respond to interaction: {}", err_msg);
    }
}
