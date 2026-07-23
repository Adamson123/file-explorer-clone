use std::sync::Arc;

use serde_json::{json, Value};

use crate::{
    commands_registry::{BoxFuture, Command},
    globals::Globals,
    user_events::{NewWindowConfig, UserEvent, WindowEvent},
    utils::{get_field_as_bool, get_field_as_string, put_value_in_result},
};

/*
 let end = match res {
                    Ok(r) => String::from(r),
                    Err(e) => String::from(e),
                };
                end
*/

// impl ToResultString for Result<(), String> {
//     fn to_result_string(self) -> Result<String, String> {
//         self.map(|_| "Ok".to_string())
//     }
// }

#[macro_export]
macro_rules! command {
    ($name:ident, |$args:ident, $globals:ident| $body:block) => {
        pub fn $name<'a>($args: &'a Value, $globals: Arc<Globals>) -> BoxFuture<'a> {
            Box::pin(async move {
                let res = { $body };
                res
            })
        }
    };
}

command!(log, |a, _g| {
    let name = get_field_as_string(&a, "name");
    put_value_in_result(&json!({"name": name}))
});

command!(minimize_window, |a, g| {
    let _ = g
        .commands_event_loop_proxy
        .lock()
        .await
        .send_event(UserEvent::WindowEvent(
            get_field_as_string(a, "window_key"),
            WindowEvent::Minimize(true),
        ));
    Ok(String::new())
});

command!(move_window, |a, g| {
    let _ = g
        .commands_event_loop_proxy
        .lock()
        .await
        .send_event(UserEvent::WindowEvent(
            get_field_as_string(a, "window_key"),
            WindowEvent::DragWindow,
        ));
    Ok(String::from("Moving window"))
});

command!(hide_decoration, |a, g| {
    let minimize = get_field_as_bool(&a, "minimize");
    let _ = g
        .commands_event_loop_proxy
        .lock()
        .await
        .send_event(UserEvent::WindowEvent(
            get_field_as_string(a, "window_key"),
            WindowEvent::HideDecoration(minimize),
        ));
    Ok(String::from("Moving window"))
});

#[macro_export]
macro_rules! command_struct {
    ($name:ident, |$args:ident, $globals:ident| $body:block) => {
        pub fn $name() -> Command {
            fn fnt<'a>($args: &'a Value, $globals: Arc<Globals>) -> BoxFuture<'a> {
                Box::pin(async move {
                    let res = { $body };
                    res
                })
            }

            let name = stringify!($name).to_string();

            Command {
                name,
                function: Arc::new(Box::new(fnt)),
            }
        }
    };
}

command_struct!(log_struct, |args, _g| {
    let name = get_field_as_string(&args, "name");
    put_value_in_result(&json!({"name": name}))
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

    // compare_yield_vs_no_yield().await;

    println!("Done reading dir: {path}");

    let entries_json = serde_json::to_string(&entries).unwrap_or(String::new());
    Ok(entries_json)
}

command_struct!(get_dir_contents, |a, g| { get_dir_c(a, g).await });

command_struct!(create_window, |a, g| {
    let window_config = NewWindowConfig {
        url: get_field_as_string(a, "url"),
        window_name: get_field_as_string(a, "window_name"),
    };

    g.event_loop_proxy
        .lock()
        .await
        .send_event(UserEvent::CreateNewWindow(window_config))
        .map_err(|a| a.to_string())?;

    Ok(String::from("Created"))
});
