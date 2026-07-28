//use std::os::raw::c_void;

use std::os::raw::c_void;

use tao::{
    dpi::LogicalSize,
    event_loop::EventLoopWindowTarget,
    platform::windows::{WindowBuilderExtWindows, WindowExtWindows},
    window::{Window, WindowBuilder},
};
use window_vibrancy::{apply_acrylic, apply_blur};

use crate::{user_events::UserEvent, webview_windows_manager::WindowConfig};

// use windows::Win32::{
//     Foundation::HWND,
//     Graphics::Gdi::{CreateRoundRectRgn, SetWindowRgn},
// };

// pub fn set_window_radius(hwnd: HWND, width: i32, height: i32) {
//     unsafe {
//         let region = CreateRoundRectRgn(
//             0, 0, width, height, 30, // corner radius X
//             30, // corner radius Y
//         );

//         SetWindowRgn(hwnd, Some(region), true);
//     }
// }

//TODO: A more structed blur effect implementation
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
        .with_inner_size(LogicalSize::new(
            window_config.width as f64,
            window_config.height as f64,
        ))
        .build(event_loop);

    match window {
        Ok(w) => {
            //   let hwnd = HWND(w.hwnd() as *mut c_void);
            // let hwnd = HWND(w.hwnd() as isize);
            //  enable_acrylic(hwnd);
            //  enable_acrylic(hwnd);
            //let _ = apply_blur(&w, Some((18, 18, 18, 220)));
            //let _ = apply_acrylic(&w, Some((18, 18, 18, 75)));
            // set_window_radius(hwnd, window_config.width, window_config.height);
            Ok(w)
        }
        Err(e) => Err(e.to_string()),
    }
}
