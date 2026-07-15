use std::collections::HashMap;

use file_explorer_clone::{
    commands::{get_dir_contents, hide_decoration, log, log_struct, minimize_window, move_window},
    commands_reg::CommandsRegistry,
    webview_init::start_webview,
};

#[tokio::main]
async fn main() {
    //  fn_future(log);
    let mut commands_reg = CommandsRegistry {
        commands: HashMap::new(),
    };

    commands_reg.register("log", Box::new(log));
    commands_reg.register("minimize_window", Box::new(minimize_window));
    commands_reg.register("move_window", Box::new(move_window));
    commands_reg.register("hide_decoration", Box::new(hide_decoration));
    commands_reg.register("get_dir_contents", Box::new(get_dir_contents));

    commands_reg.register_command(log_struct());

    start_webview(commands_reg);
}
