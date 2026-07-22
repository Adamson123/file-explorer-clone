use tao::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};

use crate::user_events::UserEvent;

pub fn create_window(window_name: &str, event_loop: &EventLoopWindowTarget<UserEvent>) -> Window {
    let window = WindowBuilder::new()
        .with_title(window_name)
        .build(event_loop)
        .unwrap();

    window
}
