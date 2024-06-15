use colored::Colorize;

use crate::commands::{Command, CommandHandler};
use crate::success;

pub struct CommandLinpeas;

impl Command for CommandLinpeas {
    fn name(&self) -> String {
        String::from("linpeas")
    }

    fn description(&self) -> String {
        format!(
            "{} {}",
            self.name(),
            "- Downloads linPEAS/winPEAS from GitHub"
        )
    }

    fn execute(&self, handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        let mut connections = handler.connections.lock().unwrap();
        match connections.send_message("LINPEAS\r\n".to_string()) {
            Ok(res) => {
                if res.starts_with("OK") {
                    let path = res.lines().nth(1).unwrap_or("NULL");
                    success!("linPEAS downloaded ({})", path.to_string());
                    #[cfg(windows)]
                    let start_cmd = path.to_string();
                    #[cfg(unix)]
                    let start_cmd =
                        format!("chmod +x {};bash {}", path.to_string(), path.to_string());
                    success!("To start linPEAS/winPEAS, open a shell using 'shell' and paste this command:\n{}", start_cmd);
                    Ok(())
                } else {
                    Err(res)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn alias(&self) -> String {
        String::from("peas")
    }
}
