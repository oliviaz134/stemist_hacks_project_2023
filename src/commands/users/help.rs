use serenity::builder::CreateApplicationCommand;

pub fn help() -> String {
    let part1: &str = "**List of available commands:**";
    let part2: &str = "`/help`: Shows a list of available commands.";
    let part3: &str = "`/userinfo`: Displays your username and points.";
    let part4: &str = "`/ask_question`: Asks a question.";
    
    format!("{}\n\t{}\n\t{}\n\t{}\n", part1, part2, part3, part4)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("Show the help message.")
}
