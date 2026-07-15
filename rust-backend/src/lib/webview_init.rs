use std::sync::Arc;

use tao::{
    event::{Event, WindowEvent},
    event_loop::EventLoopBuilder,
    window::WindowBuilder,
};

use tokio::sync::Mutex;
use wry::WebViewBuilder;

use crate::{
    commands_reg::CommandsRegistry,
    globals::Globals,
    ipc_handler::handle_ipc_msg,
    user_events::{self, UserEvent},
};

pub fn start_webview(commands_reg: CommandsRegistry) {
    //  let event_loop = EventLoop::new();

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let globals = Globals {
        event_loop_proxy: proxy,
        commands_reg,
    };
    let globals = Arc::new(Mutex::new(globals));
    let globals_clone = globals.clone();

    let window = WindowBuilder::new()
        .with_title("File explorer")
        .build(&event_loop)
        .unwrap();

    let webview = WebViewBuilder::new()
        .with_url("http://localhost:5173")
        .with_ipc_handler(move |msg| {
            let msg_clone = msg.clone();
            let globals = globals_clone.clone();

            tokio::spawn({
                async move {
                    let globals = globals.lock().await;
                    let _ = globals
                        .event_loop_proxy
                        .send_event(UserEvent::IPCMessage(msg_clone));
                }
            });
            // let globals = spawn_2_gloals_clone.lock().await;
            // let _ = globals.proxy.send_event(UserEvent::IPCMessage(msg));
        })
        .build(&window)
        .unwrap();

    //globals_clone;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = tao::event_loop::ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = tao::event_loop::ControlFlow::Exit,

            Event::UserEvent(UserEvent::IPCMessage(msg)) => {
                tokio::task::spawn({
                    let globals = globals.clone();
                    async move { handle_ipc_msg(&msg, globals).await }
                });
            }

            Event::UserEvent(UserEvent::WindowEvent(window_event)) => match window_event {
                user_events::WindowEvent::Minimize(minimize) => {
                    if minimize {
                        window.set_minimized(true);
                    } else {
                        window.set_minimized(false);
                    }
                }
                user_events::WindowEvent::DragWindow => {
                    let _ = window.drag_window();
                }
                user_events::WindowEvent::HideDecoration(hide) => {
                    window.set_decorations(!hide);
                }
            },

            Event::UserEvent(UserEvent::WebviewEvent(webview_event)) => match webview_event {
                user_events::WebviewEvent::EvaluateScript(script) => {
                    let _ = webview.evaluate_script(&script);
                }
            },

            _ => {}
        }
    })
}
