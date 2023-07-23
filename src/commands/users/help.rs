use serenity::builder::CreateApplicationCommand;
use serenity::builder::CreateEmbed;

pub fn help() -> CreateEmbed {
    let part1: &str = "**List of available commands:**";
    let part2: &str = "`/help`: Shows a list of available commands.";
    let part3: &str = "`/userinfo`: Displays your username and points.";

    let mut embed = CreateEmbed::default();
    embed
        .title("Help Menu")
        .description(format!("{}\n\t{}\n\t{}\n", part1, part2, part3))
        .color(0x00ff00);

    return embed;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("Show the help message.")
}
