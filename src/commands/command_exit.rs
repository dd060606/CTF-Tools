use std::process;

use crate::commands::{Command, CommandHandler};

pub struct CommandExit;

impl Command for CommandExit {
    fn name(&self) -> String {
        String::from("exit")
    }

    fn description(&self) -> String {
        format!(
            "{} {}",
            self.name(),
            "- Terminates the CTF-Tools console session."
        )
    }

    fn execute(&self, _handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        process::exit(0)
    }

    fn alias(&self) -> String {
        String::from("q")
    }
}
