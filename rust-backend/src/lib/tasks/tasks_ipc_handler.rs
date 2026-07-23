use std::sync::Arc;

use serde_json::{json, Value};

use crate::{
    globals::Globals,
    user_events::{UserEvent, WebviewEvent},
    utils::{construct_js_event, get_field_as_string},
};

pub fn tasks_ipc_handler(window_key: &str, body: &Value, globals: Arc<Globals>) {
    let action = get_field_as_string(body, "action");
    //Start
    if action == "start" {
        tokio::task::spawn({
            let task_name = get_field_as_string(body, "task_name");
            let id = get_field_as_string(body, "id");
            let args = body.get("args").cloned();
            let window_key = window_key.to_string();
            let globals = globals.clone();

            println!("Args ooo: {}", args.clone().unwrap());

            async move {
                let event_name = globals.tasks_manager.lock().await.start_task(
                    &task_name,
                    &window_key,
                    &args,
                    globals.clone(),
                );

                let (error, event_name) = if event_name.is_ok() {
                    ("".into(), event_name.unwrap())
                } else {
                    (event_name.err().unwrap_or(String::new()), "".into())
                };

                let response = json!({"id":id, "error": error, "data": event_name  });
                let js_event = construct_js_event("ipc-response", &response);

                // sleep(Duration::from_secs_f64(0.1)).await;
                println!("Sending back response");
                let _ = globals.event_loop_proxy.send_event(UserEvent::WebviewEvent(
                    window_key.to_string(),
                    WebviewEvent::EvaluateScript(js_event),
                ));
            }
        });
    }

    //Msg
    if action == "task_msg" {
        tokio::task::spawn({
            let event_name = get_field_as_string(body, "event_name");
            let args = body.get("args").cloned();
            let globals = globals.clone();

            async move {
                let def = json!({});
                let args = args.unwrap_or(def);
                let tasks_manager = globals.tasks_manager.lock().await;

                let sender = tasks_manager.send_msg(&event_name, &args);

                if let Some(s) = sender {
                    s.await;
                }
            }
        });
    }
}
