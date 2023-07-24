use serenity::builder::CreateApplicationCommand;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use super::super::pq::{connect, interface};

pub fn whoami(
    command_interaction: &mut ApplicationCommandInteraction,
) -> (CreateEmbed, Option<[String; 4]>, Option<[String; 4]>) {
    let user_parent = command_interaction.member.as_mut().unwrap();
    let mut connection = connect::establish_connection();
    let user_res = interface::get_user(&mut connection, user_parent.user.id.0 as i64);

    let user = match user_res {
        Ok(user) => user,
        Err(_) => {
            let _ = interface::insert_user(
                &mut connection,
                &(user_parent.user.id.0 as i64),
                &user_parent.user.name.clone(),
            );
            interface::get_user(&mut connection, user_parent.user.id.0 as i64).unwrap()
        }
    };

    let mut embed = CreateEmbed::default();
    embed
        .title("User Information")
        .description(format!(
            "Your name: {}\nYour user ID: {}\nYour points: {}\n",
            user_parent.user.name, user_parent.user.id, user.points
        ))
        .color(0xff04f);

    return (embed, None, None);
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("whoami")
        .description("See some info about you.")
}
