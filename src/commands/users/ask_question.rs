use serenity::builder::CreateApplicationCommand;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;


pub fn ask_question(_command_interaction: &mut interaction::application_command::ApplicationCommandInteraction) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("Question")
        .description("This is a test")
        .color(0xff040);

    return embed;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ask_question")
        .description("Tell the bot to ask a question to the user")
}