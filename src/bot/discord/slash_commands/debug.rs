use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::prelude::command::{CommandOptionType};
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};

use serde_json;

use crate::bot::discord::slash_commands::shared::get_database;
use crate::core::feed::{FeedCollection, FeedConfig};

pub async fn run(
    ctx: Context,
    command: &ApplicationCommandInteraction,
) -> (Context, Option<String>) {
    let options = &command.data.options;

    let option = options
        .get(0)
        .expect("Expected debug code")
        .resolved
        .as_ref()
        .expect("Expected debug code object");

    // Get database anyways
    let db_arc = get_database(&ctx).await;
    let db = db_arc.lock().unwrap();

    if let CommandDataOptionValue::String(debug_code) = option {
        match debug_code.as_ref() {
            "resync" => {
                (ctx, Some("Completed resync operation".to_string()))
            },
            _ => {
                (
                    ctx,
                    Some("Not a valid debug code. ".to_string())
                )
            }
        }
    } else {
        (
            ctx,
            Some("Please provide a valid debug code. ".to_string()),
        )
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("debug")
        .description("Developement debug code feature. Does not work in prod. ")
        .create_option(|option| {
            option
                .name("code")
                .description("Debug code. ")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
