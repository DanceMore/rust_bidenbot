use chrono::Local;

use dotenv::dotenv;
use std::env;
use tokio;

use serenity::client::{Context, EventHandler};
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::model::Permissions;
use serenity::Client;
use serenity::Error;

use serenity::async_trait;
use serenity::prelude::*;

use serenity::http::Http;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::PermissionOverwrite;
use serenity::model::prelude::RoleId;

use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::model::interactions::InteractionApplicationCommandCallbackDataFlags;

use clokwerk::Interval::Wednesday;
use clokwerk::{AsyncScheduler, Job};
//use clokwerk::Interval::Friday;
use std::time::Duration;

extern crate env_logger;
#[macro_use]
extern crate log;

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "biden" => {
                    if let Some(guild_id) = command.guild_id {
                        if let Some(guild) = guild_id.to_guild_cached(&ctx) {
                            // Now you can work with the `guild` object as expected.
                            info!("{:?} attempted to use a command...", command.user.id);
                            let guild_owner_id = guild.owner_id;

                            if command.user.id == guild_owner_id {
                                info!("{:?} appears to be a Guild Owner", guild.owner_id);
                                commands::biden::run(&command.data.options)
                            } else {
                                "You are not the server owner.".to_string()
                            }
                        } else {
                            "Server owner not found.".to_string()
                        }
                    } else {
                        "Failed to get guild information.".to_string()
                    }
                }
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .content(content)
                                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        // XXX: delete stale commands
        //let global_commands =
        //    match ApplicationCommand::get_global_application_commands(&ctx.http).await {
        //        Ok(commands) => commands,
        //        Err(why) => {
        //            eprintln!("Error getting global application commands: {:?}", why);
        //            return;
        //        }
        //    };

        //// Delete each global application command
        //for command in global_commands {
        //	debug!("[+] attempting to delete command {:?}", command);
        //    if let Err(why) =
        //        ApplicationCommand::delete_global_application_command(&ctx.http, command.id).await
        //    {
        //        eprintln!("Error deleting global application command: {:?}", why);
        //    }
        //}

        // XXX: register statements for stale commands
        //let _ = Command::create_global_application_command(&ctx, |command| {
        //    info!("attempting to register slash command for ping");
        //    commands::ping::register(command)
        //})
        //.await;

        //let _ = Command::create_global_application_command(&ctx, |command| {
        //    info!("attempting to register slash command for echo");
        //    commands::echo::register(command)
        //})
        //.await;

        let _ = Command::create_global_application_command(&ctx, |command| {
            info!("attempting to register slash command for biden");
            commands::biden::register(command)
        })
        .await;

        let mut scheduler = AsyncScheduler::new();

        // XXX: hack, we're calling 1 Job twice and detecting AM vs PM inside as our
        // logic flow conditional........ it'll be fine for now.
        //
        // I haven't done a good job passing the ctx around, so I get errors attempting
        // to use multiple scheduled event declarations.
        scheduler
            .every(Wednesday)
            .at("00:02")
            .and_every(Wednesday)
            .at("23:58")
            .run(move || toggle_everyone_send_message_permission_in_vote_channel(ctx.clone()));

        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(3000)).await;
        }

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

async fn toggle_everyone_send_message_permission_in_vote_channel(ctx: Context) {
    let http = ctx.http.clone();
    let data = ctx.data.read().await;
    debug!("inside scheduled task");

    let period = Local::now().format("%p").to_string().to_lowercase();
    let is_am = period == "am";
    let allow = is_am; // true for AM, false for PM

    //let current_minute = Local::now().minute();
    //let is_even_minute = current_minute % 2 == 0;
    //let allow = is_even_minute; // true for even minute, false for odd minute

    for guild in ctx.cache.guilds() {
        let everyone_role_result = find_everyone_role(&http, guild).await;

        let channels_result = guild.channels(&http).await;

        if let Ok(everyone_role_id) = everyone_role_result {
            if let Ok(channels) = channels_result {
                let vote_channel = channels
                    .values()
                    .find(|channel| &channel.name == "biden");

                if let Some(channel) = vote_channel {
                    info!(
                        "[+] operating on guild {} channel {} and role {}",
                        guild, channel.name, everyone_role_id
                    );
                    //channel
                    //    .say(
                    //        &http,
                    //        format!("I would operate here on {:?}.", everyone_role_id),
                    //    )
                    //    .await;

                    let mut permission_overwrite = PermissionOverwrite {
                        allow: Permissions::empty(),
                        deny: Permissions::empty(),
                        kind: serenity::model::channel::PermissionOverwriteType::Role(
                            everyone_role_id,
                        ),
                    };

                    if allow {
                        warn!("[+] opening permissions...");
                        //permission_overwrite.allow |= Permissions::SEND_MESSAGES;
                    } else {
                        warn!("[!] closing permissions...");
                        permission_overwrite.deny |= Permissions::SEND_MESSAGES;
                    }

                    // Update the channel's permissions for the @everyone role.
                    if let Err(why) = channel
                        .create_permission(&http, &permission_overwrite)
                        .await
                    {
                        error!("Error setting permissions: {:?}", why);
                    }
                } else {
                    error!("[!] vote channel not found in guild ID {}", guild);
                }
            } else {
                error!("[!] failed to fetch channels");
            }
        }
    }
}


// fun fact, GuildID == @everyone RoleID for that server....
// I might change this to triple-check it....
//
// https://docs.rs/serenity/latest/serenity/model/guild/struct.Guild.html#structfield.id
//
async fn find_everyone_role(http: impl AsRef<Http>, guild_id: GuildId) -> Result<RoleId, Error> {
    let roles = guild_id.roles(http).await?;

    for (role_id, role) in roles {
        // Check additional criteria, such as role name and position
        if role.name == "@everyone" && (role.position == -1 || role.position == 0) {
            return Ok(role_id);
        }
    }

    Err(Error::Other("Everyone role not found"))
}
