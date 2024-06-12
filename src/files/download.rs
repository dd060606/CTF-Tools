use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

use colored::Colorize;

//Check if there is an error creating the requested file
pub fn prep_download(output: &Path) -> Result<(), String> {
    if !output.exists() {
        if let Some(parent) = output.parent() {
            if !parent.exists() {
                match fs::create_dir_all(parent) {
                    Err(e) => return Err(e.to_string()),
                    _ => {}
                }
            }
        }
        match File::create(output) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    } else {
        match OpenOptions::new().write(true).create(true).open(output) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn receive_file(stream: &mut TcpStream, output: &Path, file_len: u64) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output)
        .unwrap();
    let mut buffer = [0; 1024];
    let mut received_size: u64 = 0;
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                success!("End of file reached");
                break; // End of file
            }
            Ok(n) => {
                if let Err(e) = file.write_all(&buffer[..n]) {
                    return error!("Failed to write to file: {}", e);
                }
                received_size += n as u64;
                if received_size >= file_len {
                    break;
                }
            }
            Err(e) => return error!("Failed to read from socket: {}", e),
        }
    }
}
