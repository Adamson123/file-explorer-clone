#[path = "./lib/globals.rs"]
pub mod globals;

#[path = "./lib/start_app.rs"]
pub mod start_app;

#[path = "./lib/webview_windows/create_window.rs"]
pub mod create_window;

#[path = "./lib/webview_windows/create_webview.rs"]
pub mod create_webview;

#[path = "./lib/webview_windows/webview_windows_manager.rs"]
pub mod webview_windows_manager;

#[path = "./lib/user_events/user_events.rs"]
pub mod user_events;

#[path = "./lib/user_events/user_events_handler.rs"]
pub mod user_events_handler;

#[path = "./lib/commands/commands_registry.rs"]
pub mod commands_registry;

#[path = "./lib/commands/commands.rs"]
pub mod commands;

#[path = "./lib/tasks/task_args.rs"]
pub mod task_args;

#[path = "./lib/tasks/tasks.rs"]
pub mod tasks;

#[path = "./lib/tasks/tasks_manager.rs"]
pub mod tasks_manager;

#[path = "./lib/tasks/tasks_ipc_handler.rs"]
pub mod tasks_ipc_handler;

#[path = "./lib/commands/commands_ipc_handler.rs"]
pub mod commands_ipc_handler;

#[path = "./lib/utils.rs"]
pub mod utils;
