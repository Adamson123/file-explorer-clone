use std::sync::Arc;

use tao::event_loop::EventLoopProxy;
use tokio::sync::Mutex;
//use tao::window::Window;

use crate::{commands_registry::CommandsRegistry, user_events::UserEvent};

pub struct Globals {
    // pub webview: WebView,
    // pub window: Window,
    pub event_loop_proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
    pub commands_reg: Arc<Mutex<CommandsRegistry>>,
}
