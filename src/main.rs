extern crate diesel;

use dotenv::dotenv;

use serenity::{
    async_trait,
    model::{
        application::command::Command,
        application::interaction::{Interaction, InteractionResponseType},
        gateway::Ready,
    },
    prelude::*,
};

use serenity::builder::CreateEmbed;

pub mod commands;
pub mod models;
pub mod schema;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(mut command) = interaction {
            let content = match command.data.name.as_str() {
                "help" => commands::users::help::help(),
                "ask_question" => {
                    commands::users::ask_question::ask_question(&mut command)
                }
                _ => {
                    let mut embed = CreateEmbed::default();

                    embed
                        .title("Error")
                        .description("Unknown command.")
                        .color(0xff0000);

                    embed
                }
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.add_embed(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let help_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::users::help::register(command)
        })
        .await;

        let ask_question_command =
            Command::create_global_application_command(&ctx.http, |command| {
                commands::users::ask_question::register(command)
            })
            .await;

        println!("CREATED: {:#?}\n{:#?}", help_command, ask_question_command);
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
