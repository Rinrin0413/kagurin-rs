use serenity::{model::channel::AttachmentType, builder::{CreateButton, CreateEmbed}};

/// Some interactions.
pub enum Interactions<'a> {
    /// Do some interaction with the response.
    Some(Vec<InteractMode<'a>>),
    /// Send "Not implemented yet :<".
    Dev,
    /// Do nothing.
    None,
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
