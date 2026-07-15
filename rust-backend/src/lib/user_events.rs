use wry::http::Request;

pub enum WindowEvent {
    Minimize(bool),
    DragWindow,
    HideDecoration(bool),
}

pub enum WebviewEvent {
    EvaluateScript(String),
}

pub enum UserEvent {
    IPCMessage(Request<String>),
    WindowEvent(WindowEvent),
    WebviewEvent(WebviewEvent),
}
