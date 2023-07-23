use serenity::builder::CreateApplicationCommand;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;

use super::super::pq;

pub fn leaderboard(
    _command_interaction: &mut interaction::application_command::ApplicationCommandInteraction,
) -> (CreateEmbed, Option<[String; 4]>, Option<[String; 4]>) {
    let mut conn = pq::connect::establish_connection();
    let mut users = pq::interface::get_all_users(&mut conn).unwrap_or_default();

    users.sort_by(|a, b| a.points.cmp(&b.points));

    let mut rank = String::new();
    let mut username = String::new();
    let mut points = String::new();

    for (i, user) in users.iter().rev().enumerate() {
        if i == 10 {
            break;
        }

        rank.push_str(&format!("{}\n", i + 1));
        username.push_str(&format!("{}\n", user.username));
        points.push_str(&format!("{}\n", user.points));
    }

    let mut embed = CreateEmbed::default();
    embed
        .title("Leaderboard")
        .description("The top 10 individuals on the leaderboard")
        .color(0xff040)
        .field("rank", rank, true)
        .field("username", username, true)
        .field("points", points, true);

    return (embed, None, None);
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leaderboard")
        .description("Lists the top 10 individuals on the leaderboard")
}
