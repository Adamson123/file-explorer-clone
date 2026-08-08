use std::{any::Any, collections::HashMap, sync::Arc};

use tao::{event::Event, event_loop::EventLoopBuilder};
use tokio::sync::Mutex;

use crate::{
    commands::{
        close_window, create_window, maximize_window, minimize_window, move_window, resize_window,
        restore_window, send_msg_to_window_by_selector, set_decoration,
    },
    commands_registry::{Command, CommandsRegistry},
    globals::Globals,
    start_app::{start_app, MainThreadStates, WindowEventHandler},
    states_manager::StatesManager,
    tasks_manager::{Task, TaskManager},
    user_events::{CustomEventHandler, UserEvent},
};

// pub struct RustyBridge {
//     pub commands_registry: CommandsRegistry,
//     pub task_manager: TaskManager,
// }

pub struct RustyBridgeBuilder {
    pub commands_registry: Option<CommandsRegistry>,
    pub task_manager: Option<TaskManager>,
    pub states_manager: Option<StatesManager>,
    pub url: String,
    pub custom_event_handler: Option<CustomEventHandler>,
    pub window_event_handler: Option<WindowEventHandler>,
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

        let states_manager = StatesManager { states: Vec::new() };

        Self {
            commands_registry: Some(commands_registry),
            task_manager: Some(task_manager),
            states_manager: Some(states_manager),
            custom_event_handler: Some(Box::new(move |_e, _m| {})),
            url: String::new(),
            window_event_handler: Some(Box::new(move |_e, _m| {})),
        }
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn register_commands(mut self, commands: Vec<Command>) -> Self {
        let mut commands_registry = self.commands_registry.take().unwrap();

        // Window management commands
        // commands_registry.register_command(minimize_window());
        // commands_registry.register_command(maximize_window());
        // commands_registry.register_command(restore_window());
        // commands_registry.register_command(move_window());
        // commands_registry.register_command(set_decoration());
        // commands_registry.register_command(create_window());
        // commands_registry.register_command(resize_window());
        // commands_registry.register_command(close_window());

        // Window management commands
        for c in [
            minimize_window(),
            maximize_window(),
            restore_window(),
            move_window(),
            set_decoration(),
            create_window(),
            resize_window(),
            close_window(),
            send_msg_to_window_by_selector(),
        ] {
            commands_registry.register_command(c);
        }

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

    pub fn register_states<T>(mut self, states: Vec<T>) -> Self
    where
        T: Any + Send + Sync,
    {
        let mut states_manager = self.states_manager.take().unwrap();
        for s in states {
            states_manager.add_state(s);
        }
        self.states_manager = Some(states_manager);
        self
    }

    pub fn handle_custom_event<F>(mut self, custom_event_handler: F) -> Self
    where
        F: Fn(&Box<dyn Any + Send>, &mut MainThreadStates) + 'static,
    {
        self.custom_event_handler = Some(Box::new(custom_event_handler));
        self
    }

    pub fn handle_window_event<F>(mut self, custom_event_handler: F) -> Self
    where
        F: Fn(&Event<'_, UserEvent>, &MainThreadStates) + 'static,
    {
        self.window_event_handler = Some(Box::new(custom_event_handler));
        self
    }

    pub fn start(&mut self) {
        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
        let proxy = event_loop.create_proxy();

        let globals = Globals {
            commands_register: Arc::new(Mutex::new(self.commands_registry.take().unwrap())),
            tasks_manager: Arc::new(Mutex::new(self.task_manager.take().unwrap())),
            states_manager: Arc::new(self.states_manager.take().unwrap()),
            event_loop_proxy: proxy,
        };
        let globals = Arc::new(globals);
        let custom_event_handler = Arc::new(self.custom_event_handler.take().unwrap());
        let window_event_handler = Arc::new(self.window_event_handler.take().unwrap());
        start_app(
            &self.url,
            event_loop,
            globals,
            custom_event_handler,
            window_event_handler,
        );
    }
}
