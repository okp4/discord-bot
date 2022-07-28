//! Discord bot implementations
use crate::error::{Error, ErrorKind};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{debug, warn, info};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        debug!("🤝 {} is connected!", ready.user.name);
    }
    
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                warn!("⚠️ Error sending message: {:?}", why);
            }
        }
    }
}

/// Start the discord bot (given a token)
pub async fn start(token: &str) -> Result<(), Error> {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    info!("🚀 Booting the Bot...");

    let result = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .map_err(|_| Error::from(ErrorKind::Client("Failed to create client".to_owned())));

    match result {
        Ok(mut client) => client.start().await.map_err(Error::from),
        r => r.map(|_| ()),
    }
}
