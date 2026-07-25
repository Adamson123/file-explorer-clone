use std::sync::Arc;
use tao::window::Window;
use wry::{http::Request, WebView, WebViewBuilder};

use crate::{globals::Globals, user_events::UserEvent, webview_windows_manager::WebViewConfig};

pub type IPCHandler = Arc<dyn Fn(Request<String>) + 'static>;

pub fn create_webview(
    window: &Window,
    window_key: &str,
    globals: Arc<Globals>,
    ipc_handler: &Option<IPCHandler>,
    webview_config: &WebViewConfig,
) -> Result<WebView, String> {
    let window_key = window_key.to_string();

    let ipc_handler_closure = match ipc_handler {
        Some(f) => f.clone(),
        None => Arc::new(move |_msg| {}),
    };

    const INTIALIZATION_SCRIPT: &str = r#"
        window.addEventListener("load", () => {
            console.log("Window fully reloaded");
            window.ipc.postMessage(
                JSON.stringify({
                    msg_type: "task",
                    body: {
                        action: "kill_all",
                        task_name: "",
                        task_id: "",
                        event_name: "",
                        args: {},
                        id: "",
                    },
                }),
            );
        });
     
    document.addEventListener("mousedown",(event)=>{
        if(event.target.closest(`[move-window="true"]`)){
          window.ipc.postMessage(
            JSON.stringify({
                msg_type: "command",
                body: {
                    cmd: "move_window",
                    args: {},
                    id: "",
                },
            }),
        );
         }
    });
    "#;

    let webview = WebViewBuilder::new()
        .with_url(&webview_config.url)
        .with_transparent(webview_config.transparent)
        .with_initialization_script(INTIALIZATION_SCRIPT)
        .with_ipc_handler(move |msg| {
            let msg_clone = msg.clone();
            let window_key = window_key.clone();
            //globals_clone for with_ipc_handler, and it's moved away by tokio task ❌
            // let globals_clone = Arc::clone(&globals_clone_1);
            //globals_clone for tokio task, globals_clone_1 belongs to with_ipc_handler ✔
            let globals_clone = Arc::clone(&globals);

            //Tokio spawn because ipc_handler does not accept async function
            tokio::task::spawn(async move {
                let event_loop_proxy = globals_clone.event_loop_proxy.clone();
                let _ = event_loop_proxy.send_event(UserEvent::IPCMessage(window_key, msg_clone));
            });

            ipc_handler_closure(msg);
        })
        .build(window);

    match webview {
        Ok(wv) => Ok(wv),
        Err(e) => Err(e.to_string()),
    }
}
