use wry::http::Request;

pub enum WindowEvent {
    Minimize(bool),
    DragWindow,
    HideDecoration(bool),
}

pub enum WebviewEvent {
    EvaluateScript(String),
}

pub struct NewWindowConfig {
    pub window_name: String,
    pub url: String,
}

//TODO: Maybe add Option<tokio::sync::mpsc::Sender> to some event to send back results of their execution
pub enum UserEvent {
    IPCMessage(String, Request<String>),
    CreateNewWindow(NewWindowConfig),
    WindowEvent(String, WindowEvent),
    WebviewEvent(String, WebviewEvent),
}
