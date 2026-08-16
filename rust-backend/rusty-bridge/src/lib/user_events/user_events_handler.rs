use serde_json::{json, Value};
use tao::dpi::{LogicalPosition, LogicalSize};
use tao::platform::windows::WindowExtWindows;
use windows::Win32::Foundation::{HWND, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::{
    SendMessageW, HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTLEFT, HTRIGHT, HTTOP, HTTOPLEFT,
    HTTOPRIGHT, WM_NCLBUTTONDOWN,
};

use crate::commands_ipc_handler::commands_ipc_handler;
use crate::start_app::MainThreadStates;
use crate::tasks_ipc_handler::tasks_ipc_handler;
use crate::user_events::{
    CustomEventHandler, ResizeDirection, UserEvent, WebviewEvent, WindowEvent,
};
use crate::utils::{construct_js_event, get_field_as_string};
use crate::webview_windows_manager::{WebViewWindow, WebViewWindowSetup};
use std::os::raw::c_void;
use std::sync::Arc;

pub fn user_events_handler(
    event: &UserEvent,
    main_thread_states: &mut MainThreadStates,
    custom_event_handler: Arc<CustomEventHandler>,
    _main_window_key: &str,
) {
    // let _main_webview_window = main_thread_states
    //     .webview_windows_manager
    //     .get_webview_window(main_window_key);

    let get_webview_and_unwrap = |window_key: &str| -> &WebViewWindow {
        main_thread_states
            .webview_windows_manager
            .get_webview_window(window_key)
            .unwrap()
    };

    match event {
        UserEvent::IPCMessage(window_key, selector, msg) => {
            let def = json!({});
            let msg: Value = serde_json::from_str(msg.body()).unwrap_or(def.clone());
            let msg_type = get_field_as_string(&msg, "msg_type");
            let body = msg.get("body").cloned().unwrap_or(def.clone());

            if msg_type == "task" {
                tasks_ipc_handler(window_key, &body, Arc::clone(&main_thread_states.globals));
            } else {
                commands_ipc_handler(
                    window_key,
                    selector,
                    &body,
                    Arc::clone(&main_thread_states.globals),
                );
            }
        }

        UserEvent::CreateNewWindow(w) => {
            let new_window_config = WebViewWindowSetup {
                event_loop: main_thread_states.event_loop,
                ipc_handler: None,
                window_config: w.clone(),
            };

            let _id = main_thread_states
                .webview_windows_manager
                .add_webview_window(&new_window_config);
        }

        UserEvent::WindowEvent(window_key, window_event) => {
            let webview_window = main_thread_states
                .webview_windows_manager
                .get_webview_window(window_key);

            if webview_window.is_none() {
                return;
            }

            let webview_window = webview_window.unwrap();

            match window_event {
                WindowEvent::MinimizeWindow => {
                    //  let webview_window = get_webview_and_unwrap(&window_key);
                    // if minimize.clone() {
                    //     webview_window.window.set_minimized(true);
                    // } else {
                    //     webview_window.window.set_minimized(false);
                    // }
                    let _ = webview_window.window.set_minimized(true);
                }
                WindowEvent::MaximizeWindow => {
                    let _ = webview_window.window.set_maximized(true);
                }
                WindowEvent::RestoreWindow => {
                    let _ = webview_window.window.set_maximized(false);
                }
                WindowEvent::DragWindow => {
                    //  let webview_window = get_webview_and_unwrap(&window_key);
                    let _ = webview_window.window.drag_window();
                }
                WindowEvent::HideDecoration(hide) => {
                    //   let webview_window = get_webview_and_unwrap(&window_key);
                    webview_window.window.set_decorations(!hide);
                }

                WindowEvent::CloseWindow => {
                    main_thread_states
                        .webview_windows_manager
                        .remove_webview_window(window_key);
                }

                WindowEvent::SendMsgToWindowBySelector(selector, msg) => {
                    let webview_window = main_thread_states
                        .webview_windows_manager
                        .get_webview_window_by_selector(selector);

                    if let Some(ww) = webview_window {
                        //TODO: Change event name
                        let js_event = construct_js_event("window_ipc_com", msg);
                        let _ = ww.webview.evaluate_script(&js_event);
                        // println!(
                        //     "Sent message to window with selector {}: {}",
                        //     selector, js_event
                        // );
                    } else {
                        println!(
                            "No window found with selector {} inside selectors {}. Message not sent.",
                            selector,
                            main_thread_states
                                .webview_windows_manager
                                .webview_windows
                                .values()
                                .map(|ww| ww.selector.clone())
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                    }
                }

                WindowEvent::SetVisibility(visibility) => {
                    webview_window.window.set_visible(*visibility);
                }

                WindowEvent::SetPosition(position) => {
                    if webview_window.parent_window_key.is_empty() {
                        webview_window
                            .window
                            .set_outer_position(LogicalPosition::new(position.x, position.y));
                    } else {
                        let parent = main_thread_states
                            .webview_windows_manager
                            .get_webview_window(&webview_window.parent_window_key);

                        if let Some(p) = parent {
                            let parent_outer_position = p.window.outer_position().unwrap();
                            let new_x = parent_outer_position.x + position.x as i32;
                            let new_y = parent_outer_position.y + position.y as i32;
                            webview_window
                                .window
                                .set_outer_position(LogicalPosition::new(new_x, new_y));
                        }
                    }
                }

                WindowEvent::SetSize(size) => {
                    webview_window
                        .window
                        .set_inner_size(LogicalSize::new(size.width, size.height));
                }

                //TODO: Might be removed
                WindowEvent::ResizeWindow(direction) => {
                    let ht = match direction {
                        ResizeDirection::Top => HTTOP,
                        ResizeDirection::Bottom => HTBOTTOM,
                        ResizeDirection::Left => HTLEFT,
                        ResizeDirection::Right => HTRIGHT,
                        ResizeDirection::TopLeft => HTTOPLEFT,
                        ResizeDirection::TopRight => HTTOPRIGHT,
                        ResizeDirection::BottomLeft => HTBOTTOMLEFT,
                        ResizeDirection::BottomRight => HTBOTTOMRIGHT,
                        ResizeDirection::None => return,
                    };

                    let webview_window = get_webview_and_unwrap(&window_key);

                    unsafe {
                        let hwnd = HWND(webview_window.window.hwnd() as *mut c_void);

                        let _ = ReleaseCapture();
                        SendMessageW(hwnd, WM_NCLBUTTONDOWN, Some(WPARAM(ht as usize)), None);
                    }
                }
            }
        }

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
            }
        },
        UserEvent::CustomEvent(e) => {
            custom_event_handler(e, main_thread_states);
        }
    }
}
