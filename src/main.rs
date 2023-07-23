extern crate diesel;

use dotenv::dotenv;

use serenity::{
    async_trait,
	prelude::*,
	model::{
        application::command::Command,
        application::interaction::{Interaction, InteractionResponseType},
		gateway::Ready
    }
};

pub mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(mut command) = interaction {
            let content = match command.data.name.as_str() {
                "help" => commands::users::help::help(),
                _ => "not implemented".to_string(),
            };

            if content.len() > 2000 {
                use std::fs;
                let _ = fs::create_dir("./tmp");

                fs::write("./tmp/message.txt", &content).expect("Unable to write file");

                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.add_file("./tmp/message.txt")
                            })
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
                let _ = fs::remove_file("./tmp/message.txt");
            } else {
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
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::users::help::register(command)
        }).await;

        println!("CREATED: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    dotenv().ok();
    let discord_token = std::env::var("DISCORD_ID").expect("DISCORD_ID must be set.");

    // Build our client.
    let mut client = Client::builder(discord_token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}