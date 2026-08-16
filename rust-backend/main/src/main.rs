use file_explorer_clone::{
    commands::{create_file, get_dir_contents, log_window_key, CustomEvent},
    states::{AppState, LastDirInfo},
    tasks::monitor_dir,
};
use rusty_bridge::{start::RustyBridgeBuilder, utils::construct_js_event};
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
        .handle_window_event(move |e, m| {
            match e {
                Event::WindowEvent {
                    window_id: _,
                    event: tao::event::WindowEvent::Resized(_size),
                    ..
                } => {}

                Event::WindowEvent {
                    window_id,
                    event: tao::event::WindowEvent::Focused(b),
                    ..
                } => {
                    let webview_window = m
                        .webview_windows_manager
                        .get_webview_window_by_tao_window_id(window_id);

                    if let Some(ww) = webview_window {
                        let js_event = construct_js_event(
                            "window_event",
                            &json!({ "type":"focused", "value": b }),
                        );
                        println!("Sent event value: {} to: {}", ww.selector, b);
                        let _ = ww.webview.evaluate_script(&js_event);
                    }
                }

                _ => {}
            };
        })
        .start();
}
