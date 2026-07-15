use serde_json::{json, Value};

use crate::{
    commands_reg::{BoxFuture, Command},
    globals::Globals,
    user_events::{UserEvent, WindowEvent},
    utils::{get_field_as_string, put_value_in_result},
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
        pub fn $name<'a>($args: &'a Value, $globals: &'a Globals) -> BoxFuture<'a> {
            Box::pin(async move {
                let res = { $body };
                res
            })
        }
    };
}

command!(log, |args, _g| {
    let name = get_field_as_string(&args, "name");
    put_value_in_result(&json!({"name": name}))
});

command!(minimize_window, |_a, g| {
    let _ = g
        .event_loop_proxy
        .send_event(UserEvent::WindowEvent(WindowEvent::Minimize(true)));
    Ok::<_, String>(String::new())
});

command!(move_window, |_a, g| {
    let _ = g
        .event_loop_proxy
        .send_event(UserEvent::WindowEvent(WindowEvent::DragWindow));
    Ok(String::from("Moving window"))
});

command!(hide_decoration, |_a, g| {
    let _ = g
        .event_loop_proxy
        .send_event(UserEvent::WindowEvent(WindowEvent::HideDecoration(true)));
    Ok(String::from("Moving window"))
});

command!(get_dir_contents, |a, _g| {
    let path = get_field_as_string(&a, "path");
    let mut dir_contents = tokio::fs::read_dir(path).await.map_err(|e| e.to_string())?;

    let mut entries = Vec::new();

    while let Ok(entry) = dir_contents.next_entry().await {
        if let Some(en) = entry {
            let metadata = en.metadata().await.map_err(|e| e.to_string())?;
            let path = en.path().to_string_lossy().to_string();

            let obj = json!({
                "name": en.file_name().to_string_lossy(),
                "size": metadata.len(),
                "is_dir": metadata.is_dir(),
                "path": path
            });
            entries.push(obj);
        }
    }

    let entries_json = serde_json::to_string(&entries).unwrap_or(String::new());
    Ok::<_, String>(entries_json)
});

#[macro_export]
macro_rules! command_struct {
    ($name:ident, |$args:ident, $globals:ident| $body:block) => {
        pub fn $name() -> Command {
            fn fnt<'a>($args: &'a Value, $globals: &'a Globals) -> BoxFuture<'a> {
                Box::pin(async move {
                    let res = { $body };
                    res
                })
            }

            let name = stringify!($name).to_string();

            Command {
                name,
                function: Box::new(fnt),
            }
        }
    };
}

command_struct!(log_struct, |args, _g| {
    let name = get_field_as_string(&args, "name");
    put_value_in_result(&json!({"name": name}))
});
