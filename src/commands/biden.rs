use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::interactions::application_command::ApplicationCommandOptionChoice;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(options: &[CommandDataOption]) -> String {
    if let Some(option) = options.first() {
        if let Some(value) = &option.resolved {
            if let CommandDataOptionValue::String(text) = value {
                return format!("My fellow American, this is not implemented. You requested to run: `{}`.", text).to_string();
            }
        }
    }
    "My fellow American, you must provide valid command-line arguments.".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("biden")
        .description("Open or close voting")
        .create_option(|option| {
            option
                .name("action")
                .description("Specify 'open' to open voting or 'close' to close it")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Open", "open")
                .add_string_choice("Close", "close")
        })
}
