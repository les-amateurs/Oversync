use async_trait::async_trait;
use crate::core::service::Service;

use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct DiscordBot{
    pub token: String,
    pub client: Client,
}

struct DiscordBotHandler;
#[async_trait]
impl EventHandler for DiscordBotHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "uh pong ig!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[async_trait]
impl Service for DiscordBot{
    async fn recieve(&self) {
        todo!()
    }

    async fn start(&mut self){
        let client = &mut self.client;
        client.start().await.expect("Error starting discord bot");
    }
}

impl DiscordBot {
    pub async fn new(token: String) -> DiscordBot {
        let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
        let client =  Client::builder(token.to_owned(), intents).event_handler(DiscordBotHandler).await.expect("Discord Initalize Client Failed");
        DiscordBot {
            token: token.to_owned(),
            client: client,
        }
    }
}