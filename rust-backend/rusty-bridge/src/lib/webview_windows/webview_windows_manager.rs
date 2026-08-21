use std::{collections::HashMap, os::raw::c_void, sync::Arc};

use serde::{Deserialize, Serialize};
use tao::{
    dpi::LogicalPosition,
    event_loop::EventLoopWindowTarget,
    platform::windows::WindowExtWindows,
    window::{Window, WindowId},
};
use uuid::Uuid;
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        GetWindowLongPtrW, SetWindowLongPtrW, GWLP_HWNDPARENT, GWL_EXSTYLE, GWL_STYLE, WS_BORDER,
        WS_CAPTION, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW, WS_POPUP, WS_SYSMENU, WS_THICKFRAME,
    },
};
use wry::WebView;

use crate::{
    create_webview::{create_webview, IPCHandler},
    create_window::create_window,
    globals::Globals,
    user_events::UserEvent,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebviewWindowSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebviewWindowPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WindowKind {
    App,
    Tool,
    Popup,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebViewWindowConfig {
    pub window_name: String,
    pub icon_path: String,
    pub url: String,
    pub parent_window_key: String,
    pub selector: String,
    pub position: Option<WebviewWindowPosition>,
    pub size: Option<WebviewWindowSize>,
    pub decoration: bool,
    pub transparent: bool,
    pub shadow: bool,
    pub resizable: bool,
    pub visibility: bool,
    pub kind: WindowKind,
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub window_name: String,
    pub icon_path: String,
    pub parent_window_key: String,
    pub position: Option<WebviewWindowPosition>,
    pub size: Option<WebviewWindowSize>,
    pub decoration: bool,
    pub transparent: bool,
    pub shadow: bool,
    pub resizable: bool,
    pub visibility: bool,
}

#[derive(Debug, Clone)]
pub struct WebViewConfig {
    pub url: String,
    pub transparent: bool,
}

impl WebViewWindowConfig {
    pub fn window_config(&self) -> WindowConfig {
        WindowConfig {
            window_name: self.window_name.clone(),
            size: self.size.clone(),
            decoration: self.decoration,
            // A transparent WebView2 popup can expose colored compositor pixels at
            // its rounded DOM corners. Popups are therefore always opaque at the
            // native-surface level; their page still controls the visible menu UI.
            transparent: self.transparent && !matches!(self.kind, WindowKind::Popup),
            icon_path: self.icon_path.clone(),
            shadow: self.shadow,
            resizable: self.resizable,
            parent_window_key: self.parent_window_key.clone(),
            position: self.position.clone(),
            visibility: self.visibility,
        }
    }

    pub fn webview_config(&self) -> WebViewConfig {
        WebViewConfig {
            url: self.url.clone(),
            transparent: self.transparent && !matches!(self.kind, WindowKind::Popup),
        }
    }
}

pub struct WebViewWindowSetup<'a> {
    pub webview_window_config: WebViewWindowConfig,
    pub event_loop: &'a EventLoopWindowTarget<UserEvent>,
    pub ipc_handler: Option<IPCHandler>,
}

pub struct WebViewWindow {
    pub window: Window,
    pub webview: WebView,
    pub key: String,
    pub selector: String,
    pub parent_window_key: String,
    pub kind: WindowKind,
}

pub struct WebViewWindowManager {
    pub globals: Arc<Globals>,
    pub webview_windows: HashMap<String, WebViewWindow>,
}

impl WebViewWindowManager {
    pub fn add_webview_window(
        &mut self,
        webview_window_setup: &WebViewWindowSetup,
    ) -> Result<String, String> {
        let window_key: String = Uuid::new_v4().to_string();
        let window_config: WindowConfig =
            webview_window_setup.webview_window_config.window_config();

        let window = create_window(&window_config, &webview_window_setup.event_loop);

        if let Err(e) = window {
            return Err(format!("Failed to create window: {}", e));
        }
        let window = window.unwrap();

        //Attach to parent if a parent window key is provided
        // self.attach_as_window_to_parent(&window_config, &window);
        if !window_config.parent_window_key.is_empty() {
            let parent = self.webview_windows.get(&window_config.parent_window_key);

            if let Some(p) = parent {
                match webview_window_setup.webview_window_config.kind {
                    WindowKind::App => self.attach_as_window_to_parent(&window_config, &window, p),
                    WindowKind::Tool => self.attach_as_tool_to_parent(&window_config, &window, p),
                    WindowKind::Popup => self.attach_as_popup_to_parent(&window_config, &window, p),
                }
            }
        }

        //If selector is provided, check if a window with that selector already exists, if so, return an error
        if !webview_window_setup
            .webview_window_config
            .selector
            .is_empty()
        {
            let existing_window = self
                .webview_windows
                .values()
                .find(|ww| ww.key == webview_window_setup.webview_window_config.selector);

            if let Some(existing_window) = existing_window {
                return Err(format!(
                    "A window with the selector '{}' already exists. Window key: {}",
                    webview_window_setup.webview_window_config.selector, existing_window.key
                ));
            }
        }

        let webview_config: WebViewConfig =
            webview_window_setup.webview_window_config.webview_config();
        let webview = create_webview(
            &window,
            &window_key,
            &webview_window_setup.webview_window_config.selector,
            self.globals.clone(),
            &webview_window_setup.ipc_handler,
            &webview_config,
        );

        if let Err(e) = webview {
            return Err(format!("Failed to create webview: {}", e));
        }
        let webview = webview.unwrap();

        self.webview_windows.insert(
            window_key.clone(),
            WebViewWindow {
                window,
                webview,
                key: window_key.clone(),
                parent_window_key: window_config.parent_window_key.clone(),
                selector: webview_window_setup.webview_window_config.selector.clone(),
                kind: webview_window_setup.webview_window_config.kind.clone(),
            },
        );

        Ok(window_key)
    }

    pub fn set_parent(&self, child_hwnd: isize, parent_hwnd: isize) {
        unsafe {
            SetWindowLongPtrW(
                HWND(child_hwnd as *mut c_void),
                GWLP_HWNDPARENT,
                HWND(parent_hwnd as *mut c_void).0 as isize,
            );
        }
    }

    pub fn apply_tool_window_style(&self, window: &Window) {
        unsafe {
            let hwnd = HWND(window.hwnd() as *mut c_void);
            let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            SetWindowLongPtrW(
                hwnd,
                GWL_EXSTYLE,
                (style & !(WS_EX_APPWINDOW.0 as isize)) | WS_EX_TOOLWINDOW.0 as isize,
            );
        }
    }

    pub fn apply_popup_window_style(&self, window: &Window) {
        unsafe {
            let hwnd = HWND(window.hwnd() as *mut c_void);
            let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
            // let new_style =
            //     (style & (WS_CAPTION.0 as isize | WS_SYSMENU.0 as isize)) | WS_POPUP.0 as isize;
            let new_style = (style
                & !(WS_CAPTION.0 as isize
                    | WS_SYSMENU.0 as isize
                    | WS_THICKFRAME.0 as isize
                    | WS_BORDER.0 as isize))
                | WS_POPUP.0 as isize;
            SetWindowLongPtrW(hwnd, GWL_STYLE, new_style);
        }
        self.apply_tool_window_style(window);
    }

    pub fn attach_as_tool_to_parent(
        &self,
        window_config: &WindowConfig,
        window: &Window,
        parent: &WebViewWindow,
    ) {
        let parent_hwnd = parent.window.hwnd();
        let child_hwnd = window.hwnd();

        self.apply_tool_window_style(window);
        self.set_parent(child_hwnd, parent_hwnd);

        if let Some(pos) = &window_config.position {
            self.set_position_relative_to_parent(pos, window, parent, &WindowKind::Tool);
        }
        println!(
            "Attached tool {} to parent {} ✅",
            window_config.window_name, window_config.parent_window_key
        );
    }

    pub fn attach_as_popup_to_parent(
        &self,
        window_config: &WindowConfig,
        window: &Window,
        parent: &WebViewWindow,
    ) {
        let parent_hwnd = parent.window.hwnd();
        let child_hwnd = window.hwnd();

        self.apply_popup_window_style(window);
        self.set_parent(child_hwnd, parent_hwnd);

        if let Some(pos) = &window_config.position {
            self.set_position_relative_to_parent(pos, window, parent, &WindowKind::Popup);
        }

        println!(
            "Attached popup {} to parent {} ✅",
            window_config.window_name, window_config.parent_window_key
        );
    }

    pub fn attach_as_window_to_parent(
        &self,
        window_config: &WindowConfig,
        window: &Window,
        parent: &WebViewWindow,
    ) {
        let parent_hwnd = parent.window.hwnd();
        let child_hwnd = window.hwnd();

        self.set_parent(child_hwnd, parent_hwnd);

        //Make child position relative to parent if a position is provided
        if let Some(pos) = &window_config.position {
            self.set_position_relative_to_parent(pos, window, parent, &WindowKind::App);
        }
        println!(
            "Attached window {} to parent {} ✅",
            window_config.window_name, window_config.parent_window_key
        );
    }

    pub fn set_position_relative_to_parent(
        &self,
        position: &WebviewWindowPosition,
        window: &Window,
        parent: &WebViewWindow,
        kind: &WindowKind,
    ) {
        let parent_outer_position = parent.window.outer_position().unwrap();
        let new_x = parent_outer_position.x + position.x as i32;
        let new_y = parent_outer_position.y + position.y as i32;
        window.set_outer_position(LogicalPosition::new(new_x, new_y));

        match kind {
            WindowKind::Tool => self.apply_tool_window_style(window),
            WindowKind::Popup => self.apply_popup_window_style(window),
            WindowKind::App => {}
        }
    }

    pub fn get_webview_window(&self, key: &str) -> Option<&WebViewWindow> {
        self.webview_windows.get(key)
    }

    pub fn get_webview_window_by_tao_window_id(
        &self,
        window_id: &WindowId,
    ) -> Option<&WebViewWindow> {
        self.webview_windows
            .values()
            .find(|ww| ww.window.id() == *window_id)
    }

    pub fn get_webview_window_by_selector(&self, selector: &str) -> Option<&WebViewWindow> {
        self.webview_windows.values().find(|ww| {
            if !ww.selector.is_empty() {
                ww.selector == selector
            } else {
                false
            }
        })
    }

    pub fn remove_webview_window(&mut self, key: &str) {
        self.webview_windows.remove(key);
    }
}
