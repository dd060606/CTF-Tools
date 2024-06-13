use std::process;

use colored::Colorize;

use crate::commands::{Command, CommandHandler};
use crate::error;

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

    fn execute(&self, handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        print!("\n");
        error!("Stopping CTF-Tools...");
        let mut connections = handler.connections.lock().unwrap();
        connections.send_quit();
        process::exit(0)
    }

    fn alias(&self) -> String {
        String::from("q")
    }
}
