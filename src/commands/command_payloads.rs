use colored::Colorize;

use crate::commands::{Command, CommandHandler};
use crate::payloads::platform_to_string;

pub struct CommandPayloads;

impl Command for CommandPayloads {
    fn name(&self) -> String {
        String::from("payloads")
    }

    fn description(&self) -> String {
        format!("{} {}", self.name(), "- Lists available payloads")
    }

    fn execute(&self, handler: &CommandHandler, _args: Vec<String>) -> Result<(), String> {
        println!(
            "{}",
            format!(
                "{:<name_width$}  {:<platform_width$}  {:<description_width$}",
                "NAME",
                "PLATFORM",
                "DESCRIPTION",
                name_width = 15,
                platform_width = 20,
                description_width = 50
            )
            .cyan()
        );
        println!(
            "{}",
            format!(
                "{:<name_width$}  {:<platform_width$}  {:<description_width$}",
                "----",
                "--------",
                "-----------",
                name_width = 15,
                platform_width = 20,
                description_width = 50
            )
            .cyan()
        );
        for payload in &handler.payloads.payloads {
            println!(
                "{:<name_width$}  {:<platform_width$}  {:<description_width$}",
                payload.name(),
                platform_to_string(payload.platform()),
                payload.description(),
                name_width = 15,
                platform_width = 20,
                description_width = 50
            );
        }
        Ok(())
    }

    fn alias(&self) -> String {
        String::from("payload")
    }
}
