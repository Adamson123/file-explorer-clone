use crate::{globals::Globals, user_events::UserEvent, webview_windows_manager::WebViewConfig};
use std::sync::Arc;
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    window::Window,
};
use wry::{http::Request, Rect, WebView, WebViewBuilder};

pub type IPCHandler = Arc<dyn Fn(Request<String>) + 'static>;

pub const WEBVIEW_GAP: f64 = 2.0;

pub fn create_webview(
    window: &Window,
    window_key: &str,
    selector: &str,
    globals: Arc<Globals>,
    ipc_handler: &Option<IPCHandler>,
    webview_config: &WebViewConfig,
) -> Result<WebView, String> {
    let window_key = window_key.to_string();
    let selector = selector.to_string();

    let ipc_handler_closure = match ipc_handler {
        Some(f) => f.clone(),
        None => Arc::new(move |_msg| {}),
    };

    let dynamic_script = format!(
        r#"
     localStorage.setItem("window_key", "{}");
     localStorage.setItem("selector", "{}");
    "#,
        window_key, selector
    );

    const STATIC_SCRIPT: &str = r#"
        
      
            console.log("Window fully reloaded");
            window.ipc.postMessage(
                JSON.stringify({
                    msg_type: "task",
                    body: {
                        action: "KillAll",
                        task_name: "",
                        task_id: "",
                        event_name: "",
                        args: {},
                        request_id: "",
                    },
                }),
            );
    
     
    document.addEventListener("mousedown",(event)=>{
     if(event.target.closest(`[move-window="false"]`)){
     return;
     }
        if(event.target.closest(`[move-window="true"]`)){
          window.ipc.postMessage(
            JSON.stringify({
                msg_type: "command",
                body: {
                    cmd: "move_window",
                    args: {},
                    request_id: "",
                },
            }),
        );
         }
    });
    "#;

    let initialization_script = format!("{}\n{}", dynamic_script, STATIC_SCRIPT);

    let webview = WebViewBuilder::new()
        .with_url(&webview_config.url)
        .with_transparent(webview_config.transparent)
        .with_initialization_script(initialization_script)
        .with_ipc_handler(move |msg| {
            let msg_clone = msg.clone();
            let window_key = window_key.clone();
            let selector = selector.to_string();
            let globals_clone = Arc::clone(&globals);

            //Tokio spawn because ipc_handler does not accept async function
            tokio::task::spawn(async move {
                let event_loop_proxy = globals_clone.event_loop_proxy.clone();
                let _ = event_loop_proxy
                    .send_event(UserEvent::IPCMessage(window_key, selector, msg_clone));
            });

            ipc_handler_closure(msg);
        })
        .build(window);

    match webview {
        Ok(wv) => {
            //If resizeable and decoration is false, then we need to set the webview bounds to be smaller than the window size, otherwise the webview will cover the window border and make it look like the window is not resizeable.
            if window.is_resizable() && !window.is_decorated() {
                let window_size = window.inner_size();
                let _ = wv.set_bounds(Rect {
                    position: LogicalPosition::new(WEBVIEW_GAP, WEBVIEW_GAP).into(),
                    size: LogicalSize::new(
                        window_size.width as f64 - WEBVIEW_GAP * 2.0,
                        window_size.height as f64 - WEBVIEW_GAP * 2.0,
                    )
                    .into(),
                });
            }
            Ok(wv)
        }
        Err(e) => Err(e.to_string()),
    }
}
