use std::{collections::HashMap, sync::Arc};

use file_explorer_clone::{
    commands::{
        create_window, get_dir_contents, hide_decoration, log, log_struct, minimize_window,
        move_window,
    },
    commands_registry::CommandsRegistry,
    globals::Globals,
    start_app::start_app,
    tasks_manager::TaskManager,
    user_events::UserEvent,
};
use tao::event_loop::EventLoopBuilder;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    //  fn_future(log);
    let mut commands_reg = CommandsRegistry {
        commands: HashMap::new(),
    };

    commands_reg.register("log", Arc::new(Box::new(log)));
    commands_reg.register("minimize_window", Arc::new(Box::new(minimize_window)));
    commands_reg.register("move_window", Arc::new(Box::new(move_window)));
    commands_reg.register("hide_decoration", Arc::new(Box::new(hide_decoration)));

    commands_reg.register_command(log_struct());
    commands_reg.register_command(get_dir_contents());
    commands_reg.register_command(create_window());
    //  commands_reg.register_command(monitor_dir());

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let globals = Globals {
        commands_reg: Arc::new(Mutex::new(commands_reg)),
        event_loop_proxy: Arc::new(Mutex::new(proxy)),
    };
    let globals = Arc::new(globals);

    start_app(event_loop, globals.clone());
}
