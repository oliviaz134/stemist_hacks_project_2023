use serenity::builder::CreateApplicationCommand;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;

use super::super::super::json_structs::*;

pub fn ask_question(
    _command_interaction: &mut interaction::application_command::ApplicationCommandInteraction,
) -> (CreateEmbed, Option<[String; 4]>, Option<[String; 4]>) {
    let question = parse::generate_question();

    let mut embed = CreateEmbed::default();
    embed
        .title("Question")
        .description(question.question)
        .color(0xff040)
        .field("id", format!("{}", question.id), false);

    let mut id: [String; 4] = Default::default();
    let mut choices: [String; 4] = Default::default();

    choices[0] = question.options.A;
    choices[1] = question.options.B;
    choices[2] = question.options.C;
    choices[3] = question.options.D;

    id[0] = "A".to_string();
    id[1] = "B".to_string();
    id[2] = "C".to_string();
    id[3] = "D".to_string();

    return (embed, Some(id), Some(choices));
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leaderboard")
        .description("Lists the top 10 individuals on the leaderboard")
}
