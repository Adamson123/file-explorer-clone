use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tao::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowId},
};
use uuid::Uuid;
use wry::WebView;

use crate::{
    create_webview::{create_webview, IPCHandler},
    create_window::create_window,
    globals::Globals,
    user_events::UserEvent,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebViewWindowConfig {
    pub window_name: String,
    pub icon_path: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub decoration: bool,
    pub transparent: bool,
    pub shadow: bool,
    pub resizable: bool,
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub window_name: String,
    pub icon_path: String,
    pub width: i32,
    pub height: i32,
    pub decoration: bool,
    pub transparent: bool,
    pub shadow: bool,
    pub resizable: bool,
}

#[derive(Debug, Clone)]
pub struct WebViewConfig {
    pub url: String,
    pub transparent: bool,
    pub width: i32,
    pub height: i32,
}

impl WebViewWindowConfig {
    pub fn window_config(&self) -> WindowConfig {
        WindowConfig {
            window_name: self.window_name.clone(),
            width: self.width,
            height: self.height,
            decoration: self.decoration,
            transparent: self.transparent,
            icon_path: self.icon_path.clone(),
            shadow: self.shadow,
            resizable: self.resizable,
        }
    }

    pub fn webview_config(&self) -> WebViewConfig {
        WebViewConfig {
            url: self.url.clone(),
            transparent: self.transparent,
            width: self.width,
            height: self.height,
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
        let window_config: WindowConfig = webview_window_setup.window_config.window_config();

        let window = create_window(&window_config, &webview_window_setup.event_loop);

        if let Err(e) = window {
            return Err(format!("Failed to create window: {}", e));
        }
        let window = window.unwrap();

        let webview_config: WebViewConfig = webview_window_setup.window_config.webview_config();
        let webview = create_webview(
            &window,
            &window_key,
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

    pub fn remove_webview_window(&mut self, key: &str) {
        self.webview_windows.remove(key);
    }
}
