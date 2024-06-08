use colored::Colorize;

use crate::commands::Command;

pub struct CommandList;

impl Command for CommandList {
    fn name(&self) -> String {
        String::from("list")
    }

    fn description(&self) -> String {
        format!(
            "{} {}",
            self.name().bright_blue(),
            "- List connected clients"
        )
    }

    fn execute(&self) -> Result<(), String> {
        Ok(())
    }

    fn alias(&self) -> String {
        String::from("ls")
    }
}
