use crate::ipc_handler::handle_ipc_msg;
use crate::start_app::MainThreadStates;
use crate::user_events::{UserEvent, WebviewEvent, WindowEvent};
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
            let key = window_key.clone();
            tokio::task::spawn({
                // Clone the outer `msg`.
                // The original `msg` remains owned by the outer scope.
                let msg = msg.clone();
                // The logic applies here too.
                let globals_clone = Arc::clone(&main_thread_states.globals);
                // Move the cloned String into the async task.
                async move {
                    // Borrow the task-owned String for the duration of this call.
                    handle_ipc_msg(&key, &msg, globals_clone).await
                }
            });
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
                let webview_window = main_thread_states
                    .webview_windows_manager
                    .get_webview_window(window_key);

                if let Some(webview_window) = webview_window {
                    let _ = webview_window.webview.evaluate_script(&script);
                }
            }
        },
    }
}
