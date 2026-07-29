use file_explorer_clone::{
    commands::{log_window_key, CustomEvent},
    tasks::monitor_dir,
};
use rusty_bridge::start::RustyBridgeBuilder;

#[tokio::main]
async fn main() {
    RustyBridgeBuilder::new()
        .url("http://localhost:5173")
        .register_commands(vec![log_window_key()])
        .register_tasks(vec![monitor_dir()])
        .handle_custom_event(|e, m| {
            if let Some(e) = e.downcast_ref::<CustomEvent>() {
                match *e {
                    CustomEvent::LogWindowKey => {
                        println!(
                            "I am logging the window keys from the custom event handler: {:#?}",
                            m.webview_windows_manager.webview_windows.keys()
                        );
                    }
                }
            }
        })
        .start();
}
