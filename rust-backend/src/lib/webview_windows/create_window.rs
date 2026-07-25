use tao::{
    event_loop::EventLoopWindowTarget,
    platform::windows::WindowBuilderExtWindows,
    window::{Window, WindowBuilder},
};

use crate::{user_events::UserEvent, webview_windows_manager::WindowConfig};

/*
Documentation:
Sometimes to make the background transparent, we need to set
with_transparent(true),
with_decorations(false),
with_undecorated_shadow(false),
and wry webview with_transparent(true) as well.
*/
pub fn create_window(
    window_config: &WindowConfig,
    event_loop: &EventLoopWindowTarget<UserEvent>,
) -> Result<Window, String> {
    println!("Creating window with config: {:#?}", window_config);

    let window = WindowBuilder::new()
        .with_title(window_config.window_name.clone())
        .with_transparent(window_config.transparent)
        .with_decorations(window_config.decoration)
        .with_undecorated_shadow(window_config.shadow)
        .build(event_loop);

    match window {
        Ok(w) => Ok(w),
        Err(e) => Err(e.to_string()),
    }
}
