use serde_json::Value;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct LastDirInfo {
    pub path: String,
    pub contents: Value,
    pub is_global_data: bool,
}

#[derive(Debug)]
pub struct AppState {
    pub information: Mutex<LastDirInfo>,
}
