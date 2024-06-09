use colored::Colorize;

use crate::commands::{cmd_usage, Command, CommandHandler};

pub struct CommandHelp;

impl Command for CommandHelp {
    fn name(&self) -> String {
        String::from("help")
    }

    fn description(&self) -> String {
        format!(
            "{} {} {}",
            self.name(),
            "(<command name>)",
            "- Displays a list of available commands"
        )
    }

    fn execute(&self, handler: &CommandHandler, args: Vec<String>) -> Result<(), String> {
        if args.len() > 0 {
            if let Some(cmd) = handler.get_cmd(&args[0]) {
                cmd_usage(&**cmd)
            } else {
                return Err(format!(
                    "Command '{}' not found. Use 'help' to see all available commands.",
                    args[0]
                ));
            }
        } else {
            println!("{}", "Commands\n=========".bold().cyan());
            handler
                .commands
                .iter()
                .for_each(|cmd| println!("{}", cmd.description()));
        }
        Ok(())
    }

    fn alias(&self) -> String {
        String::from("h")
    }
}
