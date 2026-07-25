use tao::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};

use crate::user_events::UserEvent;

pub fn create_window(
    window_name: &str,
    event_loop: &EventLoopWindowTarget<UserEvent>,
) -> Result<Window, String> {
    //TODO: Handle errors in window creation
    let window = WindowBuilder::new()
        .with_title(window_name)
        .build(event_loop);

    match window {
        Ok(w) => Ok(w),
        Err(e) => Err(e.to_string()),
    }
}
