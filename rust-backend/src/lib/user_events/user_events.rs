use wry::http::Request;

use crate::webview_windows_manager::WebViewWindowConfig;

pub enum WindowEvent {
    Minimize(bool),
    DragWindow,
    HideDecoration(bool),
}

pub enum WebviewEvent {
    EvaluateScript(String),
}

//TODO: Maybe add Option<tokio::sync::mpsc::Sender> to some event to send back results of their execution
pub enum UserEvent {
    IPCMessage(String, Request<String>),
    CreateNewWindow(WebViewWindowConfig),
    WindowEvent(String, WindowEvent),
    WebviewEvent(String, WebviewEvent),
}
