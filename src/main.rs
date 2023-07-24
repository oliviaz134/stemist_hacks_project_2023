extern crate diesel;

use std::sync::{Arc, Mutex};

use dotenv::dotenv;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::command::Command,
        application::interaction::{Interaction, InteractionResponseType},
        gateway::Ready,
        prelude::{component::ButtonStyle, *},
    },
    prelude::*,
};

use serenity::builder::CreateEmbed;

pub mod commands;
pub mod json_structs;
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
                "whoami" => commands::users::userinfo::whoami(&mut command),
                "leaderboard" => commands::leaderboard::leaderboard::leaderboard(&mut command),
                _ => {
                    let mut embed = CreateEmbed::default();

                    embed
                        .title("Error")
                        .description("Unknown command.")
                        .color(0xff0000);

                    (embed, None, None)
                }
            };

            let ids: Arc<Mutex<[String; 4]>> =
                Arc::new(Mutex::new(content.clone().1.unwrap_or_default()));
            let choices: Arc<Mutex<[String; 4]>> =
                Arc::new(Mutex::new(content.clone().2.unwrap_or_default()));

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.add_embed(content.0);

                            if content.1 == Default::default() || content.2 == Default::default() {
                                return message;
                            }

                            message.components(|c| {
                                c.create_action_row(|a| {
                                    a.create_button(|b| {
                                        b.custom_id(&ids.lock().unwrap()[0])
                                            .label(&choices.lock().unwrap()[0])
                                            .style(ButtonStyle::Primary)
                                    });
                                    a.create_button(|b| {
                                        b.custom_id(&ids.lock().unwrap()[1])
                                            .label(&choices.lock().unwrap()[1])
                                            .style(ButtonStyle::Primary)
                                    });
                                    a.create_button(|b| {
                                        b.custom_id(&ids.lock().unwrap()[2])
                                            .label(&choices.lock().unwrap()[2])
                                            .style(ButtonStyle::Primary)
                                    });
                                    a.create_button(|b| {
                                        b.custom_id(&ids.lock().unwrap()[3])
                                            .label(&choices.lock().unwrap()[3])
                                            .style(ButtonStyle::Primary)
                                    });

                                    return a;
                                })
                            })
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }

        if let Interaction::MessageComponent(response) = interaction.clone() {
            let mut embed = CreateEmbed::default();

            let answered_correctly = json_structs::parse::check_question(
                response.message.embeds[0].fields[0]
                    .value
                    .parse::<i64>()
                    .unwrap(),
                response.clone().data.custom_id,
            );

            if answered_correctly {
                commands::leaderboard::points::add_points(response.clone(), 1);

                embed
                    .title("Correct Answer!")
                    .description(format!("You selected: {}", response.data.custom_id))
                    .color(0x00ff00);
            } else {
                embed
                    .title("Wrong answer!")
                    .description(format!("You selected: {}", response.data.custom_id))
                    .color(0xff0000);
            }

            if let Err(why) = response
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.add_embed(embed).ephemeral(true)
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::users::help::register(command)
        })
        .await;

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::users::ask_question::register(command)
        })
        .await;

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::leaderboard::leaderboard::register(command)
        })
        .await;

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::users::userinfo::register(command)
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    json_structs::parse::parse_json_questions();

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
