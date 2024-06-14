use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

use colored::Colorize;

pub fn upload(stream: &mut TcpStream, local_path: &Path) {
    let mut file = File::open(local_path).unwrap();
    let mut file_buffer = vec![0; 1024];
    loop {
        match file.read(&mut file_buffer) {
            Ok(0) => {
                success!("File uploaded");
                break;
            }
            Ok(n) => {
                if stream.write_all(&file_buffer[..n]).is_err() {
                    error!("Error uploading file");
                    break;
                }
            }
            Err(e) => {
                error!("Error reading file {}", e.to_string());
                break;
            }
        }
    }
}
