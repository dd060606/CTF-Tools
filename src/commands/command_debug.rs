use crate::commands::{cmd_usage, Command, CommandHandler};
use crate::success;

pub struct CommandDebug;

impl Command for CommandDebug {
    fn name(&self) -> String {
        String::from("debug")
    }

    fn description(&self) -> String {
        format!(
            "{} {} {}",
            self.name(),
            "<message>",
            "- Sends a debug message to a client"
        )
    }

    fn execute(&self, handler: &CommandHandler, args: Vec<String>) -> Result<(), String> {
        if args.len() < 1 {
            cmd_usage(self);
            return Ok(());
        }
        let mut connections = handler.connections.lock().unwrap();

        let msg = format!("DEBUG\r\n{}", args.join(" ").to_string());
        match connections.send_message(msg) {
            Ok(res) => {
                success!("Received: {}", res);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn alias(&self) -> String {
        String::from("dg")
    }
}
