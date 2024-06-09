use std::io::{Read, Write};
use std::time::Duration;

use colored::Colorize;

use crate::{error, success};
use crate::commands::{cmd_usage, Command, CommandHandler};

pub struct CommandDebug;

impl Command for CommandDebug {
    fn name(&self) -> String {
        String::from("debug")
    }

    fn description(&self) -> String {
        format!(
            "{} {} {}",
            self.name(),
            "<message>",
            "- Sends a debug message to a client"
        )
    }

    fn execute(&self, handler: &CommandHandler, args: Vec<String>) -> Result<(), String> {
        if (args.len() < 1) {
            cmd_usage(self);
            return Ok(());
        }
        let mut connections = handler.connections.lock().unwrap();
        if let Some(mut stream) = connections.get_connection(1) {
            match stream.write_all(args.join(" ").as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e.to_string());
                }
            }

            // Wait for a response
            let mut buffer = [0; 1024];
            stream
                .set_read_timeout(Some(Duration::from_secs(10)))
                .unwrap();
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        error!("Connection closed by server");
                        connections.remove_connection(1);
                    } else {
                        let response = String::from_utf8_lossy(&buffer[..size]);
                        success!("Received response: {}", response);
                    }
                }
                Err(e) => {
                    error!("Failed to read from server: {}", e);
                }
            }
        }
        Ok(())
    }

    fn alias(&self) -> String {
        String::from("dg")
    }
}
