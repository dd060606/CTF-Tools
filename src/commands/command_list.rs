use colored::Colorize;

use crate::commands::{Command, CommandHandler};

pub struct CommandList;

impl Command for CommandList {
    fn name(&self) -> String {
        String::from("list")
    }

    fn description(&self) -> String {
        format!("{} {}", self.name(), "- List connected clients")
    }

    fn execute(&self, handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        println!("{}", "ID       IP".cyan());
        println!("{}", "--       --".cyan());
        //Print each connections
        let connections = handler.connections.lock().unwrap();

        for connection in &connections.connections {
            let id = *connection.0;
            let address = connection.1.peer_addr().unwrap();
            if connections.current_connection == id {
                println!(
                    "{}",
                    format!("{}        {}  (SELECTED)", id, address).bright_green()
                )
            } else {
                println!("{}        {}", id, address);
            }
        }
        Ok(())
    }

    fn alias(&self) -> String {
        String::from("ls")
    }
}
