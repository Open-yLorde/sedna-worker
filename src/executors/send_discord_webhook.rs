use discord_hook::{Embed, WebhookClient, WebhookMessage};

pub async fn send_discord_webhook(
    webhook: &str,
    title: &str,
    message: &str,
) -> Result<(), discord_hook::WebhookError> {
    // Initialize the webhook client
    let client = WebhookClient::new(webhook)?;

    let username =
        std::env::var("DISCORD_WEBHOOK_USERNAME").expect("DISCORD_WEBHOOK_USERNAME must be set");

    // Build a message with a text snippet and a rich embed
    let message = WebhookMessage::builder()
        .username(username)
        .embed(
            Embed::builder()
                .title(title)
                .description(message)
                .color(0x5865F2) // Discord Blurple
                // .field("Status", "Online", true)
                .build(),
        )
        .build()?;

    // Send the payload to Discord
    client.send(&message).await?;

    Ok(())
}
