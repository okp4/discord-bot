use actix::{ContextFutureSpawner, Handler, WrapFuture};
use serenity::{http::Http, model::id::ChannelId};
use tracing::log::{info, warn};

use crate::discord::discord_client::{
    messages::send_msg::{SendMessage, SendMessageResult},
    DiscordActor,
};

impl Handler<SendMessage> for DiscordActor {
    type Result = SendMessageResult;

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) -> Self::Result {
        let http = Http::new(&self.token);

        async move {
            info!(
                "✉️ Sending message {} to channel with ID {}",
                msg.content.clone(),
                msg.channel_id
            );
            let _ = ChannelId(msg.channel_id)
                .send_message(&http, |m| {
                    m.content(msg.content).tts(true);
                    if !msg.title.is_empty() || !msg.description.is_empty() {
                        m.embed(|e| {
                            if !msg.title.is_empty() {
                                e.title(msg.title);
                            }
                            if !msg.description.is_empty() {
                                e.description(msg.description);
                            }
                            e
                        });
                    }
                    m
                })
                .await
                .map_err(|err| warn!("Cannot send message: {:?}", err));
        }
        .into_actor(self)
        .wait(ctx);
    }
}
