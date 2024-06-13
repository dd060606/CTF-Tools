use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use colored::Colorize;

use crate::{error, success};

pub struct Connections {
    pub connections: HashMap<u16, TcpStream>,
    pub current_connection: u16,
}

impl Connections {
    pub fn new() -> Connections {
        let connections = HashMap::new();
        Connections {
            connections,
            current_connection: 1,
        }
    }
    pub fn add_connection(&mut self, id: u16, stream: TcpStream) {
        success!("New connection: {} ({})", stream.peer_addr().unwrap(), id);
        self.connections.insert(id, stream);
    }

    pub fn get_connection(&mut self, id: u16) -> Option<&mut TcpStream> {
        self.connections.get_mut(&id)
    }

    pub fn remove_connection(&mut self, id: u16) {
        error!("Connection closed ({})", &id);
        self.connections.remove(&id);
    }

    pub fn send_message(&mut self, message: String) -> Result<String, String> {
        if let Some(stream) = self.get_connection(self.current_connection) {
            match stream.write_all(message.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    self.remove_connection(self.current_connection);
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
                        self.remove_connection(1);
                        Err(String::from("Error while receiving the message!"))
                    } else {
                        let response = String::from_utf8_lossy(&buffer[..size]);
                        Ok(response.to_string())
                    }
                }
                Err(e) => Err(format!("Failed to read from server: {}", e)),
            }
        } else {
            Err(format!(
                "Connection {} not found, please change current client using 'select <id>'",
                self.current_connection
            ))
        }
    }

    //Check if the connection exists
    pub fn exists(&self, id: u16) -> bool {
        self.connections.contains_key(&id)
    }
    //Change current connection ID
    pub fn set_current_connection(&mut self, id: u16) {
        self.current_connection = id;
    }

    pub fn get_stream(&mut self) -> Option<&mut TcpStream> {
        if let Some(stream) = self.get_connection(self.current_connection) {
            Some(stream)
        } else {
            None
        }
    }

    pub fn send_quit(&mut self) {
        self.connections.iter().for_each(|mut client| {
            let _ = client.1.write_all(b"QUIT");
        });
    }
}
