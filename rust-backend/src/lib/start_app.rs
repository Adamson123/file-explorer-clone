use std::{collections::HashMap, sync::Arc};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
};

use crate::{
    globals::Globals,
    user_events::UserEvent,
    user_events_handler::user_events_handler,
    webview_windows_manager::{WebViewWindowConfig, WebViewWindowManager},
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

    let main_window_config = WebViewWindowConfig {
        event_loop: &event_loop,
        ipc_handler: None,
        url: "http://localhost:5173".into(),
        window_name: "File Explorer".into(),
    };

    let main_window_key = webview_windows_manager.add_webview_window(&main_window_config);

    event_loop.run(
        move |event: Event<'_, UserEvent>, event_loop, control_flow| {
            *control_flow = tao::event_loop::ControlFlow::Wait;

            match &event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                    ..
                } => {
                    //And why is it not complaining that they two &mut ref to webview_windows_manager, is it scope
                    // let mut main_thread_states = MainThreadStates {
                    //     event_loop: event_loop,
                    //     globals: globals_clone_2.clone(),
                    //     webview_windows_manager: &mut webview_windows_manager,
                    // };

                    let webview_window =
                        webview_windows_manager.get_webview_window_by_tao_window_id(window_id);

                    if let Some(webview_window) = webview_window {
                        let mut keys: Vec<String> = Vec::new();

                        if webview_window.key == main_window_key {
                            *control_flow = tao::event_loop::ControlFlow::Exit;

                            let windows_key: Vec<String> = webview_windows_manager
                                .webview_windows
                                .iter()
                                .map(|(v, _)| v.clone())
                                .collect();
                            keys = windows_key;
                        } else {
                            let key = webview_window.key.clone();
                            webview_windows_manager.remove_webview_window(&key);
                            keys.push(key);
                        }

                        let task_manager = globals_clone_2.clone().tasks_manager.clone();
                        tokio::task::spawn(async move {
                            task_manager.lock().await.end_multiple_window_tasks(&keys);
                        });
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
