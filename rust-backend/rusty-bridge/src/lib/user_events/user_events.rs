use std::any::Any;

use serde_json::Value;
use wry::http::Request;

use crate::{
    start_app::MainThreadStates,
    webview_windows_manager::{WebViewWindowConfig, WebviewWindowPosition, WebviewWindowSize},
};

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
    CloseWindow,
    SetFocus,
    HideDecoration(bool),
    ResizeWindow(ResizeDirection),
    SendMsgToWindowBySelector(String, Value),
    SetVisibility(bool),
    SetSize(WebviewWindowSize),
    SetPosition(WebviewWindowPosition),
}

pub enum WebviewEvent {
    EvaluateScript(String),
}

pub type CustomEventHandler = Box<dyn Fn(&Box<dyn Any + Send>, &mut MainThreadStates)>;

//TODO: Maybe add Option<tokio::sync::mpsc::Sender> to some event to send back results of their execution
pub enum UserEvent {
    IPCMessage(String, String, Request<String>),
    CreateNewWindow(WebViewWindowConfig),
    WindowEvent(String, WindowEvent),
    WebviewEvent(String, WebviewEvent),
    CustomEvent(Box<dyn Any + Send>),
}
