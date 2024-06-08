use colored::Colorize;

use crate::commands::Command;

pub struct CommandHelp;

impl Command for CommandHelp {
    fn name(&self) -> String {
        String::from("help")
    }

    fn description(&self) -> String {
        format!(
            "{} {}",
            self.name().bright_blue(),
            "- Displays a list of available commands"
        )
    }

    fn execute(&self) -> Result<(), String> {
        Ok(())
    }

    fn alias(&self) -> String {
        String::from("h")
    }
}
