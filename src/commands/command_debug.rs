use std::io::{Read, Write};

use crate::commands::{Command, CommandHandler};

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
        let mut connections = handler.connections.lock().unwrap();
        if let Some(stream) = connections.get_connection(0) {
            stream.write_all(args[0].as_bytes()).unwrap();

            // Wait for a response
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        println!("Connection ID {} closed by server", 0);
                        connections.remove_connection(0);
                    } else {
                        let response = String::from_utf8_lossy(&buffer[..size]);
                        println!("Received response from connection ID {}: {}", 0, response);
                    }
                }
                Err(e) => {
                    println!("Failed to read from connection ID {}: {}", 0, e);
                }
            }
        }
        Ok(())
    }

    fn alias(&self) -> String {
        String::from("dg")
    }
}
