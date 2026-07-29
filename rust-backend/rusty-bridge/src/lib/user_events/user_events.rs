use std::any::Any;

use wry::http::Request;

use crate::{start_app::MainThreadStates, webview_windows_manager::WebViewWindowConfig};

pub enum ResizeDirection {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None,
}

pub enum WindowEvent {
    MinimizeWindow,
    MaximizeWindow,
    RestoreWindow,
    DragWindow,
    HideDecoration(bool),
    ResizeWindow(ResizeDirection),
    CloseWindow,
}

pub enum WebviewEvent {
    EvaluateScript(String),
}

pub type CustomEventHandler = Box<dyn Fn(&Box<dyn Any + Send>, &mut MainThreadStates)>;

//TODO: Maybe add Option<tokio::sync::mpsc::Sender> to some event to send back results of their execution
pub enum UserEvent {
    IPCMessage(String, Request<String>),
    CreateNewWindow(WebViewWindowConfig),
    WindowEvent(String, WindowEvent),
    WebviewEvent(String, WebviewEvent),
    CustomEvent(Box<dyn Any + Send>),
}
