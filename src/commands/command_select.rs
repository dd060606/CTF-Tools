use crate::commands::{cmd_usage, Command, CommandHandler};
use crate::success;

pub struct CommandSelect;

impl Command for CommandSelect {
    fn name(&self) -> String {
        String::from("select")
    }

    fn description(&self) -> String {
        format!("{} {} {}", self.name(), "<id>", "- Selects a client")
    }

    fn execute(&self, handler: &CommandHandler, args: Vec<String>) -> Result<(), String> {
        if args.len() < 1 {
            cmd_usage(self);
            return Ok(());
        }
        if let Ok(id) = args[0].parse::<u16>() {
            let mut connections = handler.connections.lock().unwrap();
            if connections.exists(id) {
                connections.set_current_connection(id);
                success!("Connection {} selected", id);
                Ok(())
            } else {
                Err(format!(
                    "Connection {} does not exist, please check available connections using 'list'",
                    id
                ))
            }
        } else {
            Err(String::from("Invalid ID"))
        }
    }

    fn alias(&self) -> String {
        String::from("sel")
    }
}
