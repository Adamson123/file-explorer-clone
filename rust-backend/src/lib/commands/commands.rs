use std::sync::Arc;

use serde_json::{json, Value};

use crate::{
    command, command_struct,
    globals::Globals,
    user_events::{ResizeDirection, UserEvent, WindowEvent},
    utils::{get_field_as_bool, get_field_as_string, put_value_in_result},
    webview_windows_manager::WebViewWindowConfig,
};

command!(log, |a, _g| {
    let name = get_field_as_string(&a, "name");
    put_value_in_result(&json!({"name": name}))
});

command_struct!(minimize_window, |a, g| {
    let _ = g.event_loop_proxy.send_event(UserEvent::WindowEvent(
        get_field_as_string(a, "window_key"),
        WindowEvent::Minimize(true),
    ));
    Ok(String::new())
});

command_struct!(move_window, |a, g| {
    let _ = g.event_loop_proxy.send_event(UserEvent::WindowEvent(
        get_field_as_string(a, "window_key"),
        WindowEvent::DragWindow,
    ));
    Ok(String::from("Moving window"))
});

command_struct!(set_decoration, |a, g| {
    let decoration = get_field_as_bool(&a, "decoration");
    let _ = g.event_loop_proxy.send_event(UserEvent::WindowEvent(
        get_field_as_string(a, "window_key"),
        WindowEvent::HideDecoration(decoration),
    ));
    Ok(String::from("Moving window"))
});

command_struct!(create_window, |a, g| {
    let window_config: WebViewWindowConfig =
        serde_json::from_value(a.clone()).map_err(|e| e.to_string())?;

    g.event_loop_proxy
        .send_event(UserEvent::CreateNewWindow(window_config))
        .map_err(|a| a.to_string())?;

    Ok(String::from("Created"))
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

command_struct!(resize_window, |a, g| {
    let direction = get_field_as_string(&a, "direction");

    let direction = match direction.as_str() {
        "Left" => ResizeDirection::Left,
        "Right" => ResizeDirection::Right,
        "Top" => ResizeDirection::Top,
        "Bottom" => ResizeDirection::Bottom,
        "TopLeft" => ResizeDirection::TopLeft,
        "TopRight" => ResizeDirection::TopRight,
        "BottomLeft" => ResizeDirection::BottomLeft,
        "BottomRight" => ResizeDirection::BottomRight,
        _ => ResizeDirection::None,
    };

    let _ = g.event_loop_proxy.send_event(UserEvent::WindowEvent(
        get_field_as_string(a, "window_key"),
        WindowEvent::ResizeWindow(direction),
    ));

    Ok(String::from("Resizing window"))
});
