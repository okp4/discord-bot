use actix::{ContextFutureSpawner, Handler, WrapFuture};
use serenity::http::Http;
use tracing::log::{info, warn};

use crate::discord::discord_client::message::DiscordMessage;
use crate::discord::discord_client::{
    messages::send_msg::{SendMessage, SendMessageResult},
    DiscordActor,
};

impl<M> Handler<SendMessage<M>> for DiscordActor
where
    M: DiscordMessage + Unpin + 'static,
{
    type Result = SendMessageResult;

    fn handle(&mut self, msg: SendMessage<M>, ctx: &mut Self::Context) -> Self::Result {
        let http = Http::new(&self.token);

        async move {
            info!("✉️ Sending message to discord");
            let _ = msg
                .message
                .send_message(&http)
                .await
                .map_err(|err| warn!("Cannot send message: {:?}", err));
        }
        .into_actor(self)
        .wait(ctx);
    }
}
