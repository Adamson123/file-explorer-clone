use tao::event_loop::EventLoopProxy;
//use tao::window::Window;

use crate::{commands_reg::CommandsRegistry, user_events::UserEvent};

pub struct Globals {
    // pub webview: WebView,
    // pub window: Window,
    pub event_loop_proxy: EventLoopProxy<UserEvent>,
    pub commands_reg: CommandsRegistry,
}
