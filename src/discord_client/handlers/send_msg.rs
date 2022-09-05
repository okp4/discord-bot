use actix::{ContextFutureSpawner, Handler, WrapFuture};
use serenity::{http::Http, json::Value};
use tracing::log::{info, warn};

use crate::discord_client::{
    messages::send_msg::{SendMessage, SendMessageResult},
    DiscordActor,
};

impl Handler<SendMessage> for DiscordActor {
    type Result = SendMessageResult;

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) -> Self::Result {
        let discord_client = Http::new(&self.token);

        async move {
            info!(
                "✉️ Sending message {} to channel with ID {}",
                msg.content.clone(),
                msg.channel_id
            );
            let _ = discord_client
                .send_message(msg.channel_id, &Value::String(msg.content.clone()))
                .await
                .map_err(|err| {
                    warn!(
                        "Cannot send message {} to channel with ID {}: {:?}",
                        msg.content.clone(),
                        msg.channel_id,
                        err
                    )
                });
        }
        .into_actor(self)
        .wait(ctx);
    }
}
