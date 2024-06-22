use std::sync::{Arc, Mutex};

use colored::Colorize;

use crate::commands::{
    CommandClose, CommandDebug, CommandDownload, CommandExit, CommandGenerate, CommandHelp,
    CommandInfo, CommandList, CommandPayloads, CommandSelect, CommandShell, CommandUpload,
};
use crate::connections::Connections;
use crate::payloads::Payloads;

pub trait Command {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn execute(&self, handler: &CommandHandler, args: Vec<String>) -> Result<(), String>;

    fn alias(&self) -> String;
}

pub struct CommandHandler<'a> {
    pub commands: Vec<Box<dyn Command>>,
    pub connections: &'a Arc<Mutex<Connections>>,
    pub payloads: Payloads,
}
impl<'a> CommandHandler<'a> {
    pub fn new(connections: &'a Arc<Mutex<Connections>>) -> CommandHandler {
        let mut commands: Vec<Box<dyn Command>> = Vec::new();
        //Register commands
        commands.push(Box::new(CommandHelp {}));
        commands.push(Box::new(CommandList {}));
        commands.push(Box::new(CommandExit {}));
        commands.push(Box::new(CommandDebug {}));
        commands.push(Box::new(CommandSelect {}));
        commands.push(Box::new(CommandClose {}));
        commands.push(Box::new(CommandUpload {}));
        commands.push(Box::new(CommandDownload {}));
        commands.push(Box::new(CommandShell {}));
        commands.push(Box::new(CommandInfo {}));
        commands.push(Box::new(CommandPayloads {}));
        commands.push(Box::new(CommandGenerate {}));
        CommandHandler {
            commands,
            connections,
            payloads: Payloads::new(),
        }
    }

    pub fn handle_command(&self, command: String) -> Result<(), String> {
        match self.get_cmd(&command) {
            None => Err(format!(
                "Command '{}' not found. Use 'help' to see all available commands.",
                command
            )),
            Some(cmd) => cmd.execute(self, self.get_args(&command)),
        }
    }

    //Get command by name
    pub fn get_cmd(&self, cmd_name: &String) -> Option<&Box<dyn Command>> {
        for command in self.commands.iter() {
            if cmd_name.starts_with(&command.name()) || cmd_name.starts_with(&command.alias()) {
                return Some(command);
            }
        }
        None
    }

    //Get args of the inputted command
    fn get_args(&self, cmd: &String) -> Vec<String> {
        let mut args_res = vec![];
        let mut args = cmd.split_whitespace();
        args.next();
        for arg in args {
            args_res.push(arg.to_string());
        }
        args_res
    }
}

//Show command usage
pub fn cmd_usage(cmd: &dyn Command) {
    println!("{}", "Usage:\n------".bright_yellow());
    println!("  {}", cmd.description());
}
