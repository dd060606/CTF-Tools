use std::fs::File;
use std::path::Path;

use colored::Colorize;

use crate::commands::{cmd_usage, Command, CommandHandler};
use crate::files::upload;
use crate::success;

pub struct CommandUpload;

impl Command for CommandUpload {
    fn name(&self) -> String {
        String::from("upload")
    }

    fn description(&self) -> String {
        format!(
            "{} {} {}",
            self.name(),
            "<local path> <remote path>",
            "- Uploads a file on the target machine"
        )
    }

    fn execute(&self, handler: &CommandHandler, args: Vec<String>) -> Result<(), String> {
        if args.len() < 2 {
            cmd_usage(self);
            return Ok(());
        }
        //Check if the local file exists
        let local_path = Path::new(&args[0]);
        if !local_path.exists() && !local_path.is_file() {
            return Err(format!(
                "Local file not found: {}",
                local_path.as_os_str().to_str().unwrap_or("")
            ));
        }
        //Check if the local file is readable and get it size
        let mut file_len: u64;
        match File::open(local_path) {
            Ok(file) => match file.metadata() {
                Ok(metadata) => file_len = metadata.len(),
                Err(e) => return Err(format!("Failed to get file metadata: {}", e)),
            },
            Err(e) => return Err(e.to_string()),
        };
        let mut connections = handler.connections.lock().unwrap();
        match connections.send_message(format!("UPLOAD\n\r{}\n\r{}", args[1].to_string(), file_len))
        {
            Ok(res) => {
                if res == "OK" {
                    success!("Sending file ({} bytes)...", file_len);
                    upload(&mut connections.get_stream().unwrap(), local_path);
                    Ok(())
                } else {
                    Err(format!("Error while uploading file: {}", res))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn alias(&self) -> String {
        String::from("up")
    }
}
