use colored::Colorize;

use crate::commands::{Command, CommandHandler};

pub struct CommandInfo;

impl Command for CommandInfo {
    fn name(&self) -> String {
        String::from("info")
    }

    fn description(&self) -> String {
        format!(
            "{} {}",
            self.name(),
            "- Retrieves detailed information from the client. (OS, User, ...)"
        )
    }

    fn execute(&self, handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        let mut connections = handler.connections.lock().unwrap();

        match connections.send_message("INFO\r\n".to_string()) {
            Ok(res) => {
                println!(
                    "INFO ({})",
                    connections
                        .get_stream()
                        .unwrap()
                        .peer_addr()
                        .unwrap()
                        .ip()
                        .to_string()
                        .cyan()
                );
                println!("{}", "===================".cyan());
                println!("{}", res);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn alias(&self) -> String {
        String::from("info")
    }
}
