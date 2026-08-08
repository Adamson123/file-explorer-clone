use file_explorer_clone::{
    commands::{create_file, get_dir_contents, log_window_key, CustomEvent},
    states::{AppState, LastDirInfo},
    tasks::monitor_dir,
};
use rusty_bridge::start::RustyBridgeBuilder;
use serde_json::json;
use tao::event::Event;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let app_state = AppState {
        information: Mutex::new(LastDirInfo {
            path: String::new(),
            contents: json!({}),
            is_global_data: true,
        }),
    };

    RustyBridgeBuilder::new()
        .url("http://localhost:5173")
        .register_commands(vec![log_window_key(), get_dir_contents(), create_file()])
        .register_tasks(vec![monitor_dir()])
        .register_states(vec![app_state])
        .handle_custom_event(|e, m| {
            if let Some(event) = e.downcast_ref::<CustomEvent>() {
                match *event {
                    CustomEvent::LogWindowKey => {
                        println!(
                            "I am logging the window keys from the custom event handler: {:#?}",
                            m.webview_windows_manager.webview_windows.keys()
                        );
                    }
                }
            }
        })
        //Install Tao to handle window events
        .handle_window_event(move |e, _m| {
            match e {
                Event::WindowEvent {
                    window_id: _,
                    event: tao::event::WindowEvent::Resized(_size),
                    ..
                } => {}
                _ => {}
            };
        })
        .start();
}
