use std::sync::Arc;

use rusty_bridge::{
    command_struct, globals::Globals, user_events::UserEvent, utils::get_field_as_string,
};
use serde_json::{json, Value};

use crate::states::AppState;

pub enum CustomEvent {
    LogWindowKey,
}

command_struct!(log_window_key, |a, g| {
    let last_dir_info = g.states_manager.get_state::<AppState>();
    println!(
        "Logging window keys from the command handler and g: {:?}",
        last_dir_info
    );

    g.event_loop_proxy
        .send_event(UserEvent::CustomEvent(Box::new(CustomEvent::LogWindowKey)))
        .map_err(|e| e.to_string())?;

    Ok(String::new())
});

pub async fn get_dir_c(a: &Value, _g: Arc<Globals>) -> Result<String, String> {
    let path = get_field_as_string(&a, "path");
    let mut dir_contents = tokio::fs::read_dir(&path)
        .await
        .map_err(|e| e.to_string())?;

    let mut entries = Vec::new();

    println!("Reading path: {path}");

    while let Ok(entry_opt) = dir_contents.next_entry().await {
        let entry = match entry_opt {
            Some(e) => e,
            None => break, // End of directory.
        };

        // Try to get metadata; skip this entry if it fails.
        let metadata = match entry.metadata().await {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Skipping entry (metadata failed): {}", e);
                continue;
            }
        };

        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path().to_string_lossy().to_string();
        let is_dir = metadata.is_dir();
        let size = metadata.len();

        let obj = json!({
            "name": name,
            "size": size,
            "is_dir": is_dir,
            "path": path//.replace("\\", "\\\\"),
        });
        entries.push(obj);
    }

    println!("Done reading dir: {path}");

    let entries_json = serde_json::to_string(&entries).unwrap_or(String::new());
    Ok(entries_json)
}

command_struct!(get_dir_contents, |a, g| { get_dir_c(a, g).await });
