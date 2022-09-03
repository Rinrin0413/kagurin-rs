use serenity::builder::{CreateEmbed, CreateButton};

/// Some interactions.
/// 
/// # Variables
/// 
/// - `Some`: Some interactions.
/// - `Dev`: Send "Not implemented yet :<".
/// - `None`: Do nothing.
pub enum Interactions {
    /// Do some interaction with the response.
    Some(Vec<InteractMode>),
    /// Send "Not implemented yet :<".
    Dev,
    /// Do nothing.
    None,
}

/// The interaction mode.
/// 
/// # Variants
/// 
/// - `Message`: Add a message.
/// - `Embed`: Add a embed.
/// - `Button`: Add a button.
pub enum InteractMode {
    /// Add a message.
    Message(String),
    /// Add a embed.
    Embed(CreateEmbed),
    /// Add a button.
    Button(CreateButton),
}
