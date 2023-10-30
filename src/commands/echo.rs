//use serenity::builder::CreateApplicationCommand;
//use serenity::model::application::interaction::application_command::CommandDataOptionValue;
//use serenity::model::prelude::command::CommandOptionType;
//use serenity::model::prelude::interaction::application_command::CommandDataOption;
//
//pub fn run(options: &[CommandDataOption]) -> String {
//    if let Some(option) = options.first() {
//        if let Some(value) = &option.resolved {
//            if let CommandDataOptionValue::String(text) = value {
//                return text.clone();
//            }
//            //if let Some(text) = value.to_string() {
//            //    return text.to_string();
//            //}
//        }
//    }
//    "You didn't provide a message to echo.".to_string()
//}
//
//pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
//    command
//        .name("echo")
//        .description("Echoes a message")
//        .create_option(|option| {
//            option
//                .name("message")
//                .description("The message to echo")
//                .kind(CommandOptionType::String)
//                .required(true)
//        })
//}
