use colored::Colorize;

use crate::commands::{cmd_usage, Command, CommandHandler};
use crate::payloads::{Platform, string_to_platform};
use crate::success;

pub struct CommandGenerate;

impl Command for CommandGenerate {
    fn name(&self) -> String {
        String::from("generate")
    }

    fn description(&self) -> String {
        format!(
            "{} {} {}",
            self.name(),
            "<payload name> <platform: linux | windows> <ip> <port>",
            "- Generates a payload to execute the CTF-Tools client"
        )
    }

    fn execute(&self, handler: &CommandHandler, args: Vec<String>) -> Result<(), String> {
        if args.len() < 4 {
            cmd_usage(self);
            return Ok(());
        }
        for payload in &handler.payloads.payloads {
            if payload.name().eq_ignore_ascii_case(&args[0]) {
                let platform = string_to_platform(&args[1]).unwrap_or(Platform::Unix);
                return match payload.generate(&args[2], &args[3], platform) {
                    Ok(res) => {
                        success!(
                            "Generated payload '{}' for {} (IP={}, PORT={}):\n{}",
                            args[0],
                            args[1],
                            args[2],
                            args[3],
                            res
                        );
                        Ok(())
                    }

                    Err(e) => Err(e),
                };
            }
        }
        return Err("Payload not found. Use 'payloads' to see all available payloads.".to_string());
    }

    fn alias(&self) -> String {
        String::from("gen")
    }
}
