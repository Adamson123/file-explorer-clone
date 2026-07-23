use std::sync::Arc;

use tao::event_loop::EventLoopProxy;
use tokio::sync::Mutex;
//use tao::window::Window;

use crate::{
    commands_registry::CommandsRegistry, tasks_manager::TaskManager, user_events::UserEvent,
};

pub struct Globals {
    // pub webview: WebView,
    // pub window: Window,
    pub commands_register: Arc<Mutex<CommandsRegistry>>,
    pub tasks_manager: Arc<Mutex<TaskManager>>,

    pub event_loop_proxy: EventLoopProxy<UserEvent>,
    // pub tasks_event_loop_proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
    // pub commands_event_loop_proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
}
