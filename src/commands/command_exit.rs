use std::io::Write;
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

    fn execute(&self, handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        let connections = handler.connections.lock().unwrap();
        connections.connections.iter().for_each(|mut client| {
            let _ = client.1.write_all(b"QUIT");
        });

        process::exit(0)
    }

    fn alias(&self) -> String {
        String::from("q")
    }
}
