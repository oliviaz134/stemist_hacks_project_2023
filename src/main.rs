extern crate diesel;

use dotenv::dotenv;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::command::Command,
        application::interaction::{Interaction, InteractionResponseType},
        gateway::Ready,
        prelude::*,
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
        println!("Interaction received: {:#?}", interaction);

        if let Interaction::ApplicationCommand(mut command) = interaction.clone() {
            let content = match command.data.name.as_str() {
                "help" => commands::users::help::help(),
                "ask_question" => commands::users::ask_question::ask_question(&mut command),
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
                        .interaction_response_data(|message| {
                            message.add_embed(content).components(|c| {
                                c.create_action_row(
                                    |row: &mut serenity::builder::CreateActionRow| {
                                        row.create_select_menu(|menu| {
                                            menu.custom_id("animal_select");
                                            menu.placeholder("No animal selected");
                                            menu.options(|f| {
                                                f.create_option(|o| {
                                                    o.label("🐈 meow").value("Cat")
                                                });
                                                f.create_option(|o| {
                                                    o.label("🐕 woof").value("Dog")
                                                });
                                                f.create_option(|o| {
                                                    o.label("🐎 neigh").value("Horse")
                                                });
                                                f.create_option(|o| {
                                                    o.label("🦙 hoooooooonk").value("Alpaca")
                                                });
                                                f.create_option(|o| {
                                                    o.label("🦀 crab rave").value("Ferris")
                                                })
                                            })
                                        })
                                    },
                                )
                            })
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    
        if let Interaction::MessageComponent(response) = interaction.clone() {
            if response.data.custom_id == "animal_select" {
                let mut embed = CreateEmbed::default();

                embed
                    .title("Animal selected")
                    .description(format!("You selected: {}", response.data.values[0]))
                    .color(0x00ff00);

                if let Err(why) = response
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.add_embed(embed)
                            })
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

        let help_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::users::help::register(command)
        })
        .await;

        println!("Registered slash command: {:?}", help_command);

        let ask_question_command =
            Command::create_global_application_command(&ctx.http, |command| {
                commands::users::ask_question::register(command)
            })
            .await;

        println!("Registered slash command: {:?}", ask_question_command);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    dotenv().ok();
    let discord_token = std::env::var("DISCORD_ID").expect("DISCORD_ID must be set.");

    // Build our client.
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(discord_token, intents)
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
