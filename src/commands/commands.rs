use crate::commands::CommandHelp;

pub trait Command {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn execute(&self) -> Result<(), String>;

    fn alias(&self) -> String;
}

pub struct CommandHandler {
    commands: Vec<Box<dyn Command>>,
}
impl CommandHandler {
    pub fn new() -> CommandHandler {
        let mut commands: Vec<Box<dyn Command>> = Vec::new();
        //Register commands
        commands.push(Box::new(CommandHelp {}));
        CommandHandler { commands }
    }

    pub fn handle_command(&self, command_name: String) -> Result<(), String> {
        match self.get_cmd(&command_name) {
            None => Err(format!(
                "Command '{}' not found. Use 'help' to see all available commands.",
                command_name
            )),
            Some(cmd) => cmd.execute(),
        }
    }

    fn get_cmd(&self, cmd_name: &String) -> Option<&Box<dyn Command>> {
        for command in self.commands.iter() {
            if command.name().starts_with(cmd_name) || command.alias().starts_with(cmd_name) {
                return Some(command);
            }
        }
        None
    }
}
