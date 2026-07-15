use serde_json::Value;
use std::{collections::HashMap, future::Future, pin::Pin};

use crate::globals::Globals;
//use wry::WebView;

// pub type BoxFuture = Pin<Box<dyn Future<Output = Result<String, String>> + Send + Sync>>;

//pub type BoxFuture = Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;

//pub type BoxFuture = Pin<Box<dyn Future<Output = Result<String, String>>>>;
//pub type BoxFuture = Pin<Box<dyn Future<Output = Result<String, String>>>>;
//Arc<Mutex<Globals>>
//type FnType = dyn for<'a> Fn(&'a Value, &'a Globals) -> BoxFuture<'a> ;
pub type BoxFuture<'a> = Pin<Box<dyn Future<Output = Result<String, String>> + 'a + Send + Sync>>;
pub type FnType = Box<dyn for<'a> Fn(&'a Value, &'a Globals) -> BoxFuture<'a> + Send + Sync>;

pub struct Command {
    pub name: String,
    pub function: FnType,
}

pub struct CommandsRegistry {
    pub commands: HashMap<String, FnType>,
}

impl CommandsRegistry {
    pub fn register(&mut self, name: &str, function: FnType) {
        self.commands.insert(name.to_string(), function);
    }

    pub fn register_command(&mut self, command: Command) {
        self.commands.insert(command.name, command.function);
    }

    pub async fn invoke_command(
        &self,
        name: &str,
        args: &Value,
        globals: &Globals,
    ) -> Result<String, String> {
        let res = self.commands.get(name);
        if let Some(f) = res {
            let r = f(args, globals).await;
            return r;
            // tokio::spawn(f(args, globals));
            // Ok(String::new())
        } else {
            let msg = format!("Command {} not found", name);
            println!("{}", msg);
            return Err(msg);
        }
    }
}
