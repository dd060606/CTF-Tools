use std::{process, thread};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::{error, success};

pub fn start_client(ip: String, port: String) {
    loop {
        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(mut stream) => {
                success!("Successfully connected to server in port {}", port);
                //Wait for server
                let mut buffer = [0; 1024];
                loop {
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            if size > 0 {
                                handle_server_response(
                                    &mut stream,
                                    String::from_utf8_lossy(&buffer[..size]).to_string(),
                                );
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
        //Auto-reconnect to the server if an error occurs
        thread::sleep(Duration::from_secs(30));
    }
}

fn handle_server_response(stream: &mut TcpStream, response: String) {
    let mut lines = response.lines();
    if let Some(first_line) = lines.next() {
        match first_line {
            "CLOSE" => {
                stream.write_all(b"OK").unwrap();
                process::exit(0);
            }
            _ => stream
                .write_all(
                    format!(
                        "DEBUG -> {}",
                        lines.collect::<Vec<&str>>().join(" ").to_string()
                    )
                    .as_bytes(),
                )
                .unwrap(),
        }
    }
}
