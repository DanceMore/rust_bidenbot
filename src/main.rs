use dotenv::dotenv;
use std::env;
use tokio;

use serenity::client::{Context, EventHandler};
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::Client;

use serenity::async_trait;
use serenity::prelude::*;

use serenity::model::application::command::{Command};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};

extern crate env_logger;
#[macro_use]
extern crate log;

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            //			println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "echo" => commands::echo::run(&command.data.options),
                //			"id" => commands::id::run(&command.data.options),
                //			"attachmentinput" => commands::attachmentinput::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let _ = Command::create_global_application_command(&ctx, |command| {
            info!("attempting to register slash command for ping");
            //command.name("ping").description("A simple ping command")
	    commands::ping::register(command)
        })
        .await;

        let _ = Command::create_global_application_command(&ctx, |command| {
            info!("attempting to register slash command for echo");
	    commands::echo::register(command)
        })
        .await;

        info!("Bot is ready as {}!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    let framework = StandardFramework::new();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}
