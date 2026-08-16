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
    UI::WindowsAndMessaging::{SetWindowLongPtrW, GWLP_HWNDPARENT},
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
pub struct WebViewWindowConfig {
    pub window_name: String,
    pub icon_path: String,
    pub url: String,
    pub parent_window_key: String,
    pub selector: String,
    pub position: Option<WebviewWindowPosition>,
    pub size: Option<WebviewWindowSize>,
    // pub width: f64,
    // pub height: f64,
    pub decoration: bool,
    pub transparent: bool,
    pub shadow: bool,
    pub resizable: bool,
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
}

#[derive(Debug, Clone)]
pub struct WebViewConfig {
    pub url: String,
    pub transparent: bool,
    // pub width: f64,
    // pub height: f64,
}

impl WebViewWindowConfig {
    pub fn window_config(&self) -> WindowConfig {
        WindowConfig {
            window_name: self.window_name.clone(),
            size: self.size.clone(),
            decoration: self.decoration,
            transparent: self.transparent,
            icon_path: self.icon_path.clone(),
            shadow: self.shadow,
            resizable: self.resizable,
            parent_window_key: self.parent_window_key.clone(),
            position: self.position.clone(),
        }
    }

    pub fn webview_config(&self) -> WebViewConfig {
        WebViewConfig {
            url: self.url.clone(),
            transparent: self.transparent,
        }
    }
}

pub struct WebViewWindowSetup<'a> {
    pub window_config: WebViewWindowConfig,
    pub event_loop: &'a EventLoopWindowTarget<UserEvent>,
    pub ipc_handler: Option<IPCHandler>,
}

pub struct WebViewWindow {
    pub window: Window,
    pub webview: WebView,
    pub key: String,
    pub selector: String,
    pub parent_window_key: String,
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
        // println!(
        //     "Adding webview window with selector: {} and name: {}, obj: {:#?} ✅",
        //     webview_window_setup.window_config.selector,
        //     webview_window_setup.window_config.window_name,
        //     webview_window_setup.window_config
        // );

        let window_key: String = Uuid::new_v4().to_string();
        let window_config: WindowConfig = webview_window_setup.window_config.window_config();

        let window = create_window(&window_config, &webview_window_setup.event_loop);

        if let Err(e) = window {
            return Err(format!("Failed to create window: {}", e));
        }
        let window = window.unwrap();

        //Attach to parent if a parent window key is provided
        if !window_config.parent_window_key.is_empty() {
            let parent = self.webview_windows.get(&window_config.parent_window_key);

            if let Some(p) = parent {
                let parent_hwnd = p.window.hwnd();
                let child_hwnd = window.hwnd();

                println!(
                    "Attaching window {} to parent {}",
                    window_config.window_name, window_config.parent_window_key
                );

                unsafe {
                    SetWindowLongPtrW(
                        HWND(child_hwnd as *mut c_void),
                        GWLP_HWNDPARENT,
                        parent_hwnd,
                    );
                }

                //Make child position relative to parent if a position is provided
                if let Some(pos) = &window_config.position {
                    let parent_outer_position = p.window.outer_position().unwrap();
                    let new_x = parent_outer_position.x + pos.x as i32;
                    let new_y = parent_outer_position.y + pos.y as i32;
                    window.set_outer_position(LogicalPosition::new(new_x, new_y));
                }
            }
        }

        //If selector is provided, check if a window with that selector already exists, if so, return an error
        if !webview_window_setup.window_config.selector.is_empty() {
            let existing_window = self
                .webview_windows
                .values()
                .find(|ww| ww.key == webview_window_setup.window_config.selector);

            if let Some(existing_window) = existing_window {
                return Err(format!(
                    "A window with the selector '{}' already exists. Window key: {}",
                    webview_window_setup.window_config.selector, existing_window.key
                ));
            }
        }

        let webview_config: WebViewConfig = webview_window_setup.window_config.webview_config();
        let webview = create_webview(
            &window,
            &window_key,
            &webview_window_setup.window_config.selector,
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
                selector: webview_window_setup.window_config.selector.clone(),
            },
        );

        Ok(window_key)
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
