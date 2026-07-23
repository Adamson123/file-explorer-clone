use std::{collections::HashMap, sync::Arc};

use file_explorer_clone::{
    commands::{
        create_window, get_dir_contents, hide_decoration, log, log_struct, minimize_window,
        move_window,
    },
    commands_registry::CommandsRegistry,
    globals::Globals,
    start_app::start_app,
    tasks::monitor_dir,
    tasks_manager::TaskManager,
    user_events::UserEvent,
};
use tao::event_loop::EventLoopBuilder;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    //  fn_future(log);
    let mut commands_register = CommandsRegistry {
        commands: HashMap::new(),
    };

    commands_register.register("log", Arc::new(Box::new(log)));
    commands_register.register("minimize_window", Arc::new(Box::new(minimize_window)));
    commands_register.register("move_window", Arc::new(Box::new(move_window)));
    commands_register.register("hide_decoration", Arc::new(Box::new(hide_decoration)));

    commands_register.register_command(log_struct());
    commands_register.register_command(get_dir_contents());
    commands_register.register_command(create_window());
    //  commands_register.register_command(monitor_dir());

    let mut task_manager = TaskManager {
        task_channels: HashMap::new(),
        tasks: HashMap::new(),
    };
    task_manager.register_task(monitor_dir());

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let globals = Globals {
        commands_register: Arc::new(Mutex::new(commands_register)),
        tasks_manager: Arc::new(Mutex::new(task_manager)),
        event_loop_proxy: proxy,
    };

    let globals = Arc::new(globals);

    start_app(event_loop, globals.clone());
}
