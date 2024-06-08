use crate::commands::{Command, CommandHandler};

pub struct CommandList;

impl Command for CommandList {
    fn name(&self) -> String {
        String::from("list")
    }

    fn description(&self) -> String {
        format!("{} {}", self.name(), "- List connected clients")
    }

    fn execute(&self, _handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        Ok(())
    }

    fn alias(&self) -> String {
        String::from("ls")
    }
}
