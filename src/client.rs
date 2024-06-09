use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use colored::Colorize;

use crate::{error, success};

pub fn start_client(ip: String, port: String) {
    loop {
        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(mut stream) => {
                success!("Successfully connected to server in port {}", port);
                let mut buffer = [0; 1024];
                loop {
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            if size > 0 {
                                let response = String::from_utf8_lossy(&buffer[..size]);
                                success!("Received response from server: {}", response);
                                stream.write(b"YO").unwrap();
                            }
                        }
                        Err(e) => {
                            error!("Failed to read from server: {}", e);
                            break;
                        }
                    }
                    thread::sleep(Duration::from_millis(100)); // Prevent busy-waiting
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
        thread::sleep(Duration::from_secs(30));
    }
}
