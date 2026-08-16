use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event_loop::EventLoopWindowTarget,
    platform::windows::WindowBuilderExtWindows,
    window::{Window, WindowBuilder},
};

use crate::{user_events::UserEvent, webview_windows_manager::WindowConfig};

//TODO: Maybe add window background blur effect option
pub fn create_window(
    window_config: &WindowConfig,
    event_loop: &EventLoopWindowTarget<UserEvent>,
) -> Result<Window, String> {
    let window = WindowBuilder::new()
        .with_title(window_config.window_name.clone())
        //   .with_background_color((0, 0, 0, 0))
        .with_transparent(window_config.transparent)
        .with_decorations(window_config.decoration)
        .with_undecorated_shadow(window_config.shadow)
        .with_resizable(window_config.resizable)
        .build(event_loop);

    match window {
        Ok(w) => {
            if let Some(p) = &window_config.position {
                w.set_outer_position(LogicalPosition::new(p.x, p.y));
            }

            if let Some(s) = &window_config.size {
                w.set_inner_size(LogicalSize::new(s.width, s.height));
            }

            Ok(w)
        }
        Err(e) => Err(e.to_string()),
    }
}
