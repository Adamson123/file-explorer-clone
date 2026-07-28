use std::{collections::HashMap, sync::Arc};

use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    platform::windows::WindowExtWindows,
};

use windows::Win32::Foundation::HWND;
use wry::Rect;

use crate::{
    attach_to_desktop::attach_to_desktop,
    create_webview::WEBVIEW_GAP,
    globals::Globals,
    reserve_desktop_space::reserve_desktop_space,
    user_events::UserEvent,
    user_events_handler::user_events_handler,
    webview_windows_manager::{WebViewWindowConfig, WebViewWindowManager, WebViewWindowSetup},
};

pub struct MainThreadStates<'a> {
    pub event_loop: &'a EventLoopWindowTarget<UserEvent>,
    pub globals: Arc<Globals>,
    pub webview_windows_manager: &'a mut WebViewWindowManager,
}

pub fn start_app(event_loop: EventLoop<UserEvent>, globals: Arc<Globals>) {
    let _globals_clone = Arc::clone(&globals);
    let globals_clone_2 = Arc::clone(&globals);

    let mut webview_windows_manager = WebViewWindowManager {
        globals: globals.clone(),
        webview_windows: HashMap::new(),
    };

    let main_window_config = WebViewWindowSetup {
        event_loop: &event_loop,
        ipc_handler: None,
        window_config: WebViewWindowConfig {
            window_name: "Main Window".to_string(),
            url: "http://localhost:5173".to_string(),
            width: 900,
            height: 600,
            icon_path: String::new(),
            transparent: true,
            shadow: false,
            decoration: false,
            resizable: true,
        },
    };

    let main_window_key = webview_windows_manager.add_webview_window(&main_window_config);

    if let Err(e) = main_window_key {
        eprintln!("Failed to create main window: {}", e);
        return;
    }
    let main_window_key = main_window_key.unwrap();

    // let main_window = webview_windows_manager
    //     .get_webview_window(&main_window_key)
    //     .unwrap();

    // unsafe {
    //     let hwnd = HWND(main_window.window.hwnd() as *mut std::ffi::c_void);
    //     attach_to_desktop(hwnd).unwrap();
    // }

    event_loop.run(
        move |event: Event<'_, UserEvent>, event_loop, control_flow| {
            *control_flow = tao::event_loop::ControlFlow::Wait;

            match &event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                    ..
                } => {
                    let webview_window =
                        webview_windows_manager.get_webview_window_by_tao_window_id(window_id);

                    if let Some(webview_window) = webview_window {
                        let mut keys: Vec<String> = Vec::new();

                        if webview_window.key == main_window_key {
                            *control_flow = tao::event_loop::ControlFlow::Exit;

                            let windows_key: Vec<String> = webview_windows_manager
                                .webview_windows
                                .keys()
                                .map(|k| k.clone())
                                .collect();

                            keys = windows_key;
                        } else {
                            let key = webview_window.key.clone();
                            webview_windows_manager.remove_webview_window(&key);
                            keys.push(key);
                        }

                        let task_manager = globals_clone_2.clone().tasks_manager.clone();
                        tokio::task::spawn(async move {
                            task_manager.lock().await.end_multiple_window_tasks(&keys)
                        });
                    }
                }

                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    if let Some(webview) =
                        webview_windows_manager.get_webview_window_by_tao_window_id(window_id)
                    {
                        //If resizeable and decoration is false, then we need to set the webview bounds to be smaller than the window size, otherwise the webview will cover the window border and make it look like the window is not resizeable.

                        if webview.window.is_resizable() && !webview.window.is_decorated() {
                            webview
                                .webview
                                .set_bounds(Rect {
                                    position: LogicalPosition::new(WEBVIEW_GAP, WEBVIEW_GAP).into(),
                                    size: LogicalSize::new(
                                        size.width as f64 - WEBVIEW_GAP * 2.0,
                                        size.height as f64 - WEBVIEW_GAP * 2.0,
                                    )
                                    .into(),
                                })
                                .unwrap();
                        }

                        //   let hwnd = HWND(webview.window.hwnd() as *mut std::ffi::c_void);
                        //  set_window_radius(hwnd, size.width as i32, size.height as i32);
                    }
                }

                Event::UserEvent(e) => {
                    let mut main_thread_states = MainThreadStates {
                        event_loop: event_loop,
                        globals: globals_clone_2.clone(),
                        webview_windows_manager: &mut webview_windows_manager,
                    };

                    user_events_handler(&e, &mut main_thread_states, &main_window_key);
                }
                _ => {}
            }
        },
    );
}
