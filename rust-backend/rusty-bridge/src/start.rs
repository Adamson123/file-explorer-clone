use std::{collections::HashMap, sync::Arc};

use tao::event_loop::EventLoopBuilder;
use tokio::sync::Mutex;

use crate::{
    commands::{
        close_window, create_window, minimize_window, move_window, resize_window, set_decoration,
    },
    commands_registry::{Command, CommandsRegistry},
    globals::Globals,
    start_app::start_app,
    tasks_manager::{Task, TaskManager},
    user_events::UserEvent,
};

// pub struct RustyBridge {
//     pub commands_registry: CommandsRegistry,
//     pub task_manager: TaskManager,
// }

pub struct RustyBridgeBuilder {
    pub commands_registry: Option<CommandsRegistry>,
    pub task_manager: Option<TaskManager>,
    pub url: String,
}

//TODO: Allow to add custom user event listeners
//TODO: Allow to attach custom event handles

impl RustyBridgeBuilder {
    pub fn new() -> Self {
        let commands_registry = CommandsRegistry {
            commands: HashMap::new(),
        };

        let task_manager = TaskManager {
            active_tasks: HashMap::new(),
            tasks: HashMap::new(),
        };

        Self {
            commands_registry: Some(commands_registry),
            task_manager: Some(task_manager),
            url: String::new(),
        }
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn register_commands(mut self, commands: Vec<Command>) -> Self {
        let mut commands_registry = self.commands_registry.take().unwrap();
        // Window management commands
        commands_registry.register_command(minimize_window());
        commands_registry.register_command(move_window());
        commands_registry.register_command(set_decoration());
        commands_registry.register_command(create_window());
        commands_registry.register_command(resize_window());
        commands_registry.register_command(close_window());

        for c in commands {
            commands_registry.register_command(c);
        }

        self.commands_registry = Some(commands_registry);

        self
    }

    pub fn register_tasks(mut self, tasks: Vec<Task>) -> Self {
        for t in tasks {
            if let Some(ref mut tm) = self.task_manager {
                tm.register_task(t);
            }
        }
        self
    }

    pub fn start(&mut self) {
        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
        let proxy = event_loop.create_proxy();

        let globals = Globals {
            commands_register: Arc::new(Mutex::new(self.commands_registry.take().unwrap())),
            tasks_manager: Arc::new(Mutex::new(self.task_manager.take().unwrap())),
            event_loop_proxy: proxy,
        };
        let globals = Arc::new(globals);
        start_app(&self.url, event_loop, globals);
    }
}
