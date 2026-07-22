use std::{collections::HashMap, sync::Arc};

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

pub struct WebViewWindowConfig<'a> {
    pub window_name: String,
    pub url: String,
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
    pub fn add_webview_window(&mut self, webview_window_config: &WebViewWindowConfig) -> String {
        let id = Uuid::new_v4().to_string();
        let window = create_window(
            &webview_window_config.window_name,
            &webview_window_config.event_loop,
        );

        let webview = create_webview(
            &window,
            &id,
            self.globals.clone(),
            webview_window_config.ipc_handler.clone(),
            &webview_window_config.url,
        );

        self.webview_windows.insert(
            id.clone(),
            WebViewWindow {
                window,
                webview,
                key: id.clone(),
            },
        );

        id
    }

    pub fn get_webview_window(&self, key: &str) -> Option<&WebViewWindow> {
        self.webview_windows.get(key)
    }

    pub fn get_webview_window_by_tao_window_id(&self, id: &WindowId) -> Option<&WebViewWindow> {
        self.webview_windows
            .values()
            .find(|ww| ww.window.id() == *id)
    }

    pub fn remove_webview_window(&mut self, key: &str) {
        self.webview_windows.remove(key);
    }
}
