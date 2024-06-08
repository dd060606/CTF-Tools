use colored::Colorize;

use crate::commands::{Command, CommandHandler};

pub struct CommandHelp;

impl Command for CommandHelp {
    fn name(&self) -> String {
        String::from("help")
    }

    fn description(&self) -> String {
        format!(
            "{} {}",
            self.name(),
            "- Displays a list of available commands"
        )
    }

    fn execute(&self, handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        println!("{}", "Commands\n=========".bold().cyan());
        handler
            .commands
            .iter()
            .for_each(|cmd| println!("{}", cmd.description()));
        Ok(())
    }

    fn alias(&self) -> String {
        String::from("h")
    }
}
