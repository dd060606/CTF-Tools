use crate::commands::{Command, CommandHandler};

pub struct CommandClose;

impl Command for CommandClose {
    fn name(&self) -> String {
        String::from("close")
    }

    fn description(&self) -> String {
        format!(
            "{} {}",
            self.name(),
            "- Stops the CTF-Tools process on the selected client"
        )
    }

    fn execute(&self, handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        let mut connections = handler.connections.lock().unwrap();
        let id = connections.current_connection;
        match connections.send_message("CLOSE\r\n".to_string()) {
            Ok(_) => {
                connections.remove_connection(id);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn alias(&self) -> String {
        String::from("kill")
    }
}
