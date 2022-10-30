use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::prelude::command::{CommandOptionType, Command};
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue, ApplicationCommandInteraction,
};

use serde_json;

use crate::core::feed::{FeedConfig, FeedCollection};
use crate::bot::discord::slash_commands::shared::get_database;

pub async fn run(ctx: Context, command: &ApplicationCommandInteraction) -> (Context, Option<String>) {
    
    let options = &command.data.options;

    let option = options
        .get(0)
        .expect("Expected attachment option")
        .resolved
        .as_ref()
        .expect("Expected attachment object");

    if let CommandDataOptionValue::Attachment(attachment) = option {
        let file_result = attachment.download().await;
        if let Ok(file) = file_result {
            let str = String::from_utf8(file).unwrap_or("{}".to_string());
            let mut status = "".to_owned();
            match serde_json::from_str::<FeedConfig>(&str) {
                Ok(feed_config) => {
                    let db_arc = get_database(&ctx).await;
                    let db = db_arc.lock().unwrap();
                    // reusing this quite a lot
                    let mut save_feed_collection = |feed_collection: FeedCollection, key: &str| {
                        match db.put(key,command.guild_id.unwrap().to_string().as_ref(),&feed_collection) {
                            Ok(_) => {
                                status.push_str(format!("`{}` was updated \n", key).as_ref());
                            },
                            Err(_) => {
                                status.push_str(format!("`{}` was failed to update \n", key).as_ref());
                            },
                        }
                    };
                    match feed_config.hourly {
                        Some(feed_collection) => {
                            save_feed_collection(feed_collection,"hourly");
                        },
                        None => {},
                    }
                    match feed_config.daily {
                        Some(feed_collection) => {
                            save_feed_collection(feed_collection,"daily");
                        },
                        None => {},
                    }
                    match feed_config.weekly {
                        Some(feed_collection) => {
                            save_feed_collection(feed_collection,"weekly");
                        },
                        None => {},
                    }

                    (ctx, Some(format!("Updated. {} bytes transferred. {} ", attachment.size, status)))
                }
                Err(error) => {
                    (ctx, Some(format!("Invalid format. {}",error)))
                }
            }
        }else{
            // todo add: debug data printouts
            (ctx, Some(format!("File download error. ")))
        }
        
    } else {
        (ctx, Some("Please provide a valid configuration file (not found). ".to_string()))
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("configure").description("Configure feeds for this guild. ").create_option(
        |option| {
            option
                .name("attachment")
                .description("Configuration file. Json is supported at the moment. ")
                .kind(CommandOptionType::Attachment)
                .required(true)
        },
    )
}