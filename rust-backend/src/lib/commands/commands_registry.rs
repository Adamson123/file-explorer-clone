use serde_json::Value;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use crate::globals::Globals;

pub type BoxFuture<'a> = Pin<Box<dyn Future<Output = Result<String, String>> + 'a + Send>>;
pub type CommandFnType =
    Arc<Box<dyn for<'a> Fn(&'a Value, Arc<Globals>) -> BoxFuture<'a> + Send + Sync>>;

pub struct Command {
    pub name: String,
    pub function: CommandFnType,
}

pub struct CommandsRegistry {
    pub commands: HashMap<String, CommandFnType>,
}

impl CommandsRegistry {
    pub fn register(&mut self, name: &str, function: CommandFnType) {
        self.commands.insert(name.to_string(), function);
    }

    pub fn register_command(&mut self, command: Command) {
        self.commands.insert(command.name, command.function);
    }

    pub fn invoke_command<'a>(
        &self,
        name: &str,
        args: &'a Value,
        globals: Arc<Globals>,
    ) -> Result<BoxFuture<'a>, String> {
        let f = self.get_command(name);
        match f {
            Some(f) => Ok(f(args, globals)),
            None => Err(format!("Command {} not found", name)),
        }
    }

    pub fn get_command(&self, name: &str) -> Option<CommandFnType> {
        self.commands.get(name).cloned()
    }
}
