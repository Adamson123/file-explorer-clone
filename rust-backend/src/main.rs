use std::{collections::HashMap, sync::Arc};

use file_explorer_clone::{
    commands::{
        create_window, get_dir_contents, log, minimize_window, move_window, set_decoration,
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

    commands_register.register("log", Arc::new(log));

    // Window management commands
    commands_register.register_command(minimize_window());
    commands_register.register_command(move_window());
    commands_register.register_command(set_decoration());
    commands_register.register_command(create_window());

    commands_register.register_command(get_dir_contents());

    let mut task_manager = TaskManager {
        active_tasks: HashMap::new(),
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
