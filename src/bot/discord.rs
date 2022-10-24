mod slash_commands;

use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::Command;
use crate::core::service::Service;
use crate::core::db::Database;

use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct DiscordBot{
    pub token: String,
    pub client: Client,
    pub database: Arc<std::sync::Mutex<Database>>,
}


struct DatabaseInTypeMap;

impl TypeMapKey for DatabaseInTypeMap {
    type Value = Arc<Mutex<Database>>;
}

struct DiscordBotHandler;

impl DiscordBotHandler {
    async fn get_database(&self, ctx: &Context) -> Arc<Mutex<Database>>{
        let type_map = ctx.data.read().await;
        let db_arc = type_map.get::<DatabaseInTypeMap>().unwrap().clone();
        db_arc
    }
}

#[async_trait]
impl EventHandler for DiscordBotHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!db_path" {
            // Testing accessing the database through a message commasnd
            let meta_pathbuf = self.get_database(&ctx).await.lock().unwrap().get_meta_path();
            let test = meta_pathbuf.to_str().unwrap();
            if let Err(why) = msg.channel_id.say(&ctx.http, test).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let (ctx, message): (Context, Option<String>) = match command.data.name.as_str() {
                "configure" => {
                    slash_commands::configure::run(ctx, &command).await
                }
                _ => {
                    let error_msg = format!("Not implemented. Requested {}", command.data.name.as_str());
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content(error_msg))
                        })
                        .await
                    {
                        println!("Slash command error response failure: {}", why);
                    }
                    (ctx, None)
                }
            };

            match message {
                Some(message_str) => {
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content(message_str))
                        })
                        .await
                    {
                        println!("Slash command normal response failure: {}", why);
                    }
                }
                None => todo!("Don't"),
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        Command::create_global_application_command(&ctx.http, |command| {
            command.name("test").description("Example")
        }).await.expect("Commands should be synced. ");
        Command::create_global_application_command(&ctx.http, |command| {
            slash_commands::configure::register(command)
        }).await.expect("Commands should be synced. ");
        println!("Command sync done");
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
    pub async fn new(database_arc: Arc<std::sync::Mutex<Database>>, token: String) -> DiscordBot {
        let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
        let client =  Client::builder(token.to_owned(), intents).event_handler(DiscordBotHandler).await.expect("Discord Initalize Client Failed");
        client.data.write().await.insert::<DatabaseInTypeMap>(database_arc.clone());
        DiscordBot {
            token: token.to_owned(),
            client: client,
            database: database_arc, // we own the arc now!
        }
    }
}