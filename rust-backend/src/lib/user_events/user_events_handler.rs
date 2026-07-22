use serde_json::{json, Value};

use crate::commands_ipc_handler::commands_ipc_handler;
use crate::start_app::MainThreadStates;
use crate::tasks_ipc_handler::tasks_ipc_handler;
use crate::user_events::{UserEvent, WebviewEvent, WindowEvent};
use crate::utils::get_field_as_string;
use crate::webview_windows_manager::WebViewWindowConfig;
use std::sync::Arc;

pub fn user_events_handler(
    event: &UserEvent,
    main_thread_states: &mut MainThreadStates,
    main_window_key: &str,
) {
    let _main_webview_window = main_thread_states
        .webview_windows_manager
        .get_webview_window(main_window_key)
        .unwrap();

    match event {
        UserEvent::IPCMessage(window_key, msg) => {
            let def = json!({});
            let body: Value = serde_json::from_str(msg.body()).unwrap_or(def.clone());
            let msg_type = get_field_as_string(&body, "msg_type");

            // println!("Msg Type: {msg_type}");

            if msg_type == "task" {
                tasks_ipc_handler(window_key, &body, main_thread_states.globals.clone());
            } else {
                let key = window_key.clone();
                //TODO: make commands_ipc_handler sync
                tokio::task::spawn({
                    // Clone the outer `msg`.
                    // The original `msg` remains owned by the outer scope.
                    let body = body.clone();
                    // The logic applies here too.
                    let globals_clone = Arc::clone(&main_thread_states.globals);
                    // Move the cloned String into the async task.
                    async move {
                        // Borrow the task-owned String for the duration of this call.
                        commands_ipc_handler(&key, &body, globals_clone).await
                    }
                });
            }
        }

        UserEvent::CreateNewWindow(c) => {
            let new_window_config = WebViewWindowConfig {
                event_loop: main_thread_states.event_loop,
                ipc_handler: None,
                url: c.url.clone(),
                window_name: c.window_name.clone(),
            };

            let _id = main_thread_states
                .webview_windows_manager
                .add_webview_window(&new_window_config);
        }

        UserEvent::WindowEvent(window_key, window_event) => match window_event {
            WindowEvent::Minimize(minimize) => {
                let webview_window = main_thread_states
                    .webview_windows_manager
                    .get_webview_window(window_key);

                if let Some(webview_window) = webview_window {
                    if minimize.clone() {
                        webview_window.window.set_minimized(true);
                    } else {
                        webview_window.window.set_minimized(false);
                    }
                }
            }
            WindowEvent::DragWindow => {
                let webview_window = main_thread_states
                    .webview_windows_manager
                    .get_webview_window(window_key);

                if let Some(webview_window) = webview_window {
                    let _ = webview_window.window.drag_window();
                }
            }
            WindowEvent::HideDecoration(hide) => {
                let webview_window = main_thread_states
                    .webview_windows_manager
                    .get_webview_window(window_key);

                if let Some(webview_window) = webview_window {
                    webview_window.window.set_decorations(!hide);
                }
            }
        },
        UserEvent::WebviewEvent(window_key, webview_event) => match webview_event {
            WebviewEvent::EvaluateScript(script) => {
                //If main_window is not expecting any reply, this will not reflect there.
                //So send it to the window that is expectiing it !!!
                // let _ = main_webview_window.webview.evaluate_script(&script);
                if window_key == "*" {
                    for (_key, value) in &main_thread_states.webview_windows_manager.webview_windows
                    {
                        let _ = value.webview.evaluate_script(&script);
                    }
                } else {
                    let webview_window = main_thread_states
                        .webview_windows_manager
                        .get_webview_window(window_key);

                    if let Some(webview_window) = webview_window {
                        let _ = webview_window.webview.evaluate_script(&script);
                    }
                }

                //TODO: Each tasks should store window id for the window from which they are called from, so they only send event only to the window
            }
        },
    }
}
