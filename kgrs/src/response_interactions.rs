use serenity::builder::CreateEmbed;

/// The interaction mode.
/// 
/// # Variants
/// 
/// - `Message`: Send a message.
/// - `Embed`: Send a embed.
/// - `Dev`: Send "Not implemented yet :<".
/// - `None`: Do nothing.
pub enum InteractMode {
    /// Send a message.
    Message(String),
    /// Send a embed.
    Embed(CreateEmbed),
    /// Send "Not implemented yet :<".
    Dev,
    /// Do nothing.
    None,
}
