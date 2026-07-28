use windows::{
    core::Result,
    Win32::{
        Foundation::RECT,
        UI::WindowsAndMessaging::{
            GetSystemMetrics, SystemParametersInfoW, SM_CXSCREEN, SM_CYSCREEN, SPIF_SENDCHANGE,
            SPI_SETWORKAREA,
        },
    },
};

pub unsafe fn reserve_desktop_space(reserved_width: i32) -> Result<()> {
    // 1. Get full screen dimensions
    let screen_width = GetSystemMetrics(SM_CXSCREEN);
    let screen_height = GetSystemMetrics(SM_CYSCREEN);

    // 2. Define the new "Work Area" (The safe zone for icons/windows)
    // We subtract 'reserved_width' from the right side.
    let mut new_work_area = RECT {
        left: 0,
        top: 0,
        right: screen_width - reserved_width, // Shrink the right edge
        bottom: screen_height,
    };

    // 3. Tell Windows to update the workspace
    // SPIF_SENDCHANGE notifies all apps (Explorer) to redraw and move icons
    SystemParametersInfoW(
        SPI_SETWORKAREA,
        0,
        Some(&mut new_work_area as *mut _ as *mut std::ffi::c_void),
        SPIF_SENDCHANGE,
    )?;

    println!("Desktop work area updated. Icons should shift left.");
    Ok(())
}
