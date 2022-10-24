use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::prelude::command::{CommandOptionType, Command};
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue, ApplicationCommandInteraction,
};

pub fn run(ctx: Context, command: &ApplicationCommandInteraction) -> (Context, Option<String>) {
    
    let options = &command.data.options;

    let option = options
        .get(0)
        .expect("Expected attachment option")
        .resolved
        .as_ref()
        .expect("Expected attachment object");

    if let CommandDataOptionValue::Attachment(attachment) = option {
        (ctx, Some(format!("Attachment name: {}, attachment size: {}", attachment.filename, attachment.size)))
    } else {
        (ctx, Some("Please provide a valid attachment".to_string()))
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