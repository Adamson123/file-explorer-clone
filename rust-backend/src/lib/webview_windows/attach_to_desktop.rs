// // use windows::core::{BOOL, PCSTR};
// // use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
// // use windows::Win32::UI::WindowsAndMessaging::{
// //     EnumWindows, FindWindowA, FindWindowExA, GetWindowLongPtrA, SendMessageA, SetParent,
// //     SetWindowLongPtrA, SetWindowPos, ShowWindow, GWL_STYLE, HWND_BOTTOM, SWP_NOACTIVATE,
// //     SWP_NOMOVE, SWP_NOSIZE, SW_SHOW, WS_CAPTION, WS_CHILD, WS_CLIPSIBLINGS, WS_MAXIMIZEBOX,
// //     WS_MINIMIZEBOX, WS_SYSMENU, WS_THICKFRAME, WS_VISIBLE,
// // };

// // /// Find the "WorkerW" window that holds the desktop icons (SHELLDLL_DefView).
// // fn find_desktop_workerw() -> Option<HWND> {
// //     unsafe {
// //         // 1. Find Progman (root desktop). Result<HWND> in windows 0.61+
// //         let progman = FindWindowA(PCSTR(b"Progman\0".as_ptr()), None).ok()?;

// //         // 2. Trigger creation of the real WorkerW (idempotent)
// //         let _ = SendMessageA(progman, 0x052C, WPARAM(0), LPARAM(0));

// //         let mut result: Option<HWND> = None;

// //         // Callback signature: unsafe extern "system" fn(HWND, LPARAM) -> BOOL
// //         unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
// //             let result = &mut *(lparam.0 as *mut Option<HWND>);

// //             // Check class name "WorkerW"
// //             let mut class = [0u8; 64];
// //             let len = windows::Win32::UI::WindowsAndMessaging::GetClassNameA(hwnd, &mut class);
// //             if len == 0 {
// //                 return BOOL::from(true);
// //             }
// //             let class_name = std::str::from_utf8(&class[..len as usize]).unwrap_or("");
// //             if class_name != "WorkerW" {
// //                 return BOOL::from(true);
// //             }

// //             // Check if this WorkerW contains a SHELLDLL_DefView child
// //             // FindWindowExA expects Option<HWND> for the first arg
// //             let defview = FindWindowExA(
// //                 Some(hwnd),
// //                 None,
// //                 PCSTR(b"SHELLDLL_DefView\0".as_ptr()),
// //                 None,
// //             );
// //             match defview {
// //                 Ok(h) if !h.is_invalid() => {
// //                     *result = Some(hwnd);
// //                     BOOL::from(false) // stop
// //                 }
// //                 _ => BOOL::from(true),
// //             }
// //         }

// //         // EnumWindows expects: fn(HWND, LPARAM) -> BOOL, LPARAM
// //         let _ = EnumWindows(Some(enum_callback), LPARAM(&mut result as *mut _ as isize));
// //         result
// //     }
// // }

// // /// Attach a window (by HWND) to the desktop background, behind all icons.
// // pub fn attach_to_desktop(hwnd: HWND) {
// //     unsafe {
// //         if let Some(worker) = find_desktop_workerw() {
// //             // Reparent to the desktop worker
// //             SetParent(hwnd, Some(worker));

// //             // Remove unwanted window styles and make it a child
// //             let mut style = GetWindowLongPtrA(hwnd, GWL_STYLE) as u32;
// //             style &= !(WS_CAPTION.0
// //                 | WS_THICKFRAME.0
// //                 | WS_MINIMIZEBOX.0
// //                 | WS_MAXIMIZEBOX.0
// //                 | WS_SYSMENU.0);
// //             style |= WS_CHILD.0 | WS_CLIPSIBLINGS.0 | WS_VISIBLE.0;
// //             SetWindowLongPtrA(hwnd, GWL_STYLE, style as isize);

// //             // Place at the bottom of the z-order (behind icons)
// //             SetWindowPos(
// //                 hwnd,
// //                 Some(HWND_BOTTOM),
// //                 0,
// //                 0,
// //                 0,
// //                 0,
// //                 SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
// //             );

// //             // Show the window
// //             ShowWindow(hwnd, SW_SHOW);
// //         }
// //     }
// // }

// // use windows::Win32::Graphics::Gdi::{
// //     GetMonitorInfoA, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
// // };
// // use windows::Win32::UI::WindowsAndMessaging::{HWND_TOP, SWP_SHOWWINDOW};

// // pub fn attach_to_desktop_with_fullscreen(hwnd: HWND) {
// //     unsafe {
// //         attach_to_desktop(hwnd); // your existing function

// //         // Get monitor dimensions where the window will appear
// //         let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
// //         let mut monitor_info = MONITORINFO::default();
// //         monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
// //         GetMonitorInfoA(monitor, &mut monitor_info).ok();

// //         let rc_work = monitor_info.rcWork; // work area (excludes taskbar)
// //         let x = rc_work.left;
// //         let y = rc_work.top;
// //         let width = rc_work.right - rc_work.left;
// //         let height = rc_work.bottom - rc_work.top;

// //         // Place the window covering the entire work area, behind icons
// //         SetWindowPos(
// //             hwnd,
// //             Some(HWND_TOP), // keep behind icons? Actually HWND_BOTTOM is already set, we don't need to change
// //             0,
// //             0, // use HWND_BOTTOM but set pos/size
// //             width,
// //             height,
// //             SWP_SHOWWINDOW, // now it's visible and sized
// //         );

// //         ShowWindow(hwnd, SW_SHOW);
// //     }
// // }

// use windows::core::{w, Error, Result, HRESULT};
// use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
// use windows::Win32::UI::WindowsAndMessaging::*;

// pub unsafe fn attach_to_desktop(hwnd: HWND) -> Result<()> {
//     let progman = FindWindowW(w!("Progman"), None).unwrap_or_default();

//     if progman.is_invalid() {
//         return Err(Error::new(
//             HRESULT(0x80004005u32 as i32),
//             "Progman not found",
//         ));
//     }
// GetDes
//     // Force WorkerW creation
//     SendMessageTimeoutW(
//         progman,
//         0x052C,
//         WPARAM(0),
//         LPARAM(0),
//         SMTO_NORMAL,
//         1000,
//         None,
//     );

//     let mut workerw = HWND::default();

//     unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
//         // Look for desktop icon host
//         let shell_view =
//             FindWindowExW(Some(hwnd), None, w!("SHELLDLL_DefView"), None).unwrap_or_default();

//         if !shell_view.is_invalid() {
//             // Find the WorkerW after the desktop view
//             let worker = FindWindowExW(None, Some(hwnd), w!("WorkerW"), None).unwrap_or_default();

//             if !worker.is_invalid() {
//                 println!("WorkerW found: {:?}", worker);

//                 *(lparam.0 as *mut HWND) = worker;

//                 return BOOL(0);
//             }
//         }

//         BOOL(1)
//     }

//     EnumWindows(Some(enum_proc), LPARAM(&mut workerw as *mut HWND as isize))?;

//     if workerw.is_invalid() {
//         return Err(Error::new(
//             HRESULT(0x80004005u32 as i32),
//             "WorkerW not found",
//         ));
//     }

//     println!("Attaching to {:?}", workerw);

//     SetParent(hwnd, Some(workerw))?;

//     Ok(())
// }

// use windows::{
//     core::*,
//     Win32::{
//         Foundation::{HWND, LPARAM, WPARAM},
//         UI::WindowsAndMessaging::{
//             EnumWindows, FindWindowW, GetDesktopWindow, SendMessageTimeoutW, SendMessageW,
//             SetParent, SetWindowPos, HWND_BOTTOM, HWND_TOP, SMTO_NORMAL, SWP_NOSIZE,
//         },
//     },
// };

// static mut WORKERW_HWND: HWND = HWND(ptr::null_mut());

// pub unsafe fn attach_to_desktop(hwnd: HWND) -> Result<()> {
//     let progman = FindWindowW(w!("Progman"), None);
//     if progman.is_err() {
//         return Err(Error::from_win32()); // Progman not found (shouldn't happen)
//     } else {
//         println!("Found progman: {:#?}", progman.clone().unwrap());
//     }
//     let progman = progman.unwrap();

//     // 2. Send the special message to create a WorkerW behind Progman
//     //    0x052C = WM_SPAWN_WORKER (undocumented but stable)
//     let mut result: usize = 0;
//     SendMessageTimeoutW(
//         progman,
//         0x052C,
//         WPARAM(0),
//         LPARAM(0),
//         SMTO_NORMAL,
//         1000,
//         Some(&mut result),
//     );
//     // if result.e == 0 {
//     //     println!(
//     //         "Error creating workerw behind progman: {}",
//     //         Error::from_win32().to_string()
//     //     );
//     //     return Err(Error::from_win32());
//     // }
//     let _ = EnumWindows(Some(), LPARAM(0));
//     //let workerw = find_w

//     let parent = GetDesktopWindow();

//     let res = SetParent(hwnd, Some(parent));

//     if let Err(e) = res {
//         println!("Error setting parent: {}", e.to_string());
//     } else if let Ok(_) = res {
//         println!("Set parent successfully");
//     };

//     let res = SetWindowPos(hwnd, Some(HWND_BOTTOM), 10, 10, 0, 0, SWP_NOSIZE);

//     if let Err(e) = res {
//         println!("Error setting position: {}", e.to_string());
//     } else if let Ok(_) = res {
//         println!("Set position successfully");
//     };

//     Ok(())
// }

use std::ptr;
use windows::{
    core::{w, Error, Result, BOOL, PCWSTR},
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{
            EnumWindows, FindWindowExW, FindWindowW, GetClassNameW, GetWindowLongPtrW,
            SendMessageTimeoutW, SetParent, SetWindowLongPtrW, SetWindowPos, GWL_STYLE, HWND_TOP,
            SMTO_NORMAL, SWP_NOMOVE, SWP_NOSIZE, WS_CHILD, WS_VISIBLE,
        },
    },
};

// Global handles to track the desktop shell structure
static mut SHELL_DLL_DEFVIEW: HWND = HWND(ptr::null_mut());
static mut SHELL_DLL_PARENT: HWND = HWND(ptr::null_mut());

pub unsafe fn attach_to_desktop(hwnd: HWND) -> Result<()> {
    // 1. Find Progman (Program Manager)
    let progman = FindWindowW(w!("Progman"), PCWSTR::null())?;

    // 2. Send 0x052C to Progman. This spawns the WorkerW background mechanism if it's not there.
    let mut result: usize = 0;
    let _ = SendMessageTimeoutW(
        progman,
        0x052C,
        WPARAM(0),
        LPARAM(0),
        SMTO_NORMAL,
        1000,
        Some(&mut result),
    );

    // 3. Hunt for the parent window that actually contains the "SHELLDLL_DefView" (the desktop icons)
    SHELL_DLL_DEFVIEW = HWND(ptr::null_mut());
    SHELL_DLL_PARENT = HWND(ptr::null_mut());

    let _ = EnumWindows(Some(enum_windows_callback), LPARAM(0));

    if SHELL_DLL_PARENT.is_invalid() {
        // Fallback: If we couldn't find the WorkerW parent, try using Progman directly
        // This usually happens if the wallpaper hasn't been separated yet
        if let Ok(shell) =
            FindWindowExW(Some(progman), None, w!("SHELLDLL_DefView"), PCWSTR::null())
        {
            if !shell.is_invalid() {
                SHELL_DLL_PARENT = progman;
                SHELL_DLL_DEFVIEW = shell;
            }
        }
    }

    if SHELL_DLL_PARENT.is_invalid() {
        println!("Error: Could not locate desktop shell parent.");
        return Err(Error::from_win32());
    }

    // 4. Update your window style to be a Child window (mandatory for SetParent to work reliably here)
    let current_style = GetWindowLongPtrW(hwnd, GWL_STYLE);
    SetWindowLongPtrW(
        hwnd,
        GWL_STYLE,
        (current_style as u32 | WS_CHILD.0 | WS_VISIBLE.0) as isize,
    );

    // 5. Attach your window to the Desktop Shell Parent (Progman or WorkerW)
    SetParent(hwnd, Some(SHELL_DLL_PARENT))?;

    // 6. KEY FIX: Move window to TOP of the Z-order inside this group
    // This places it *above* the icons (SHELLDLL_DefView) so it catches clicks,
    // but because the whole group is the desktop, it stays behind all other apps.
    SetWindowPos(hwnd, Some(HWND_TOP), 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE)?;

    println!("Widget attached successfully to {:?}!", SHELL_DLL_PARENT);
    Ok(())
}

// Callback to find which window holds the SHELLDLL_DefView
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, _: LPARAM) -> BOOL {
    // We are looking for a window (WorkerW or Progman) that has SHELLDLL_DefView as a child
    if let Ok(shell_view) = FindWindowExW(Some(hwnd), None, w!("SHELLDLL_DefView"), PCWSTR::null())
    {
        if !shell_view.is_invalid() {
            SHELL_DLL_PARENT = hwnd;
            SHELL_DLL_DEFVIEW = shell_view;
            return false.into(); // Found it, stop searching
        }
    }
    true.into()
}
