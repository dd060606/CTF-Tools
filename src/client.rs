use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use colored::Colorize;

pub fn start_client(ip: String, port: String) {
    loop {
        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(mut stream) => {
                println!(
                    "[{}] Successfully connected to server in port {}",
                    "+".bright_green(),
                    port
                );

                let new_connection = b"NEW";

                let _ = stream.write_all(new_connection);

                let mut buffer = [0; 1024];
                loop {
                    match stream.read_exact(&mut buffer) {
                        Ok(_) => {
                            let response = String::from_utf8_lossy(&buffer);
                            println!("Received response from server: {}", response);
                        }
                        Err(e) => {
                            println!("Failed to read from server: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("[{}] Error: {}", "-".red(), e);
            }
        }
        thread::sleep(Duration::from_secs(30))
    }
}
