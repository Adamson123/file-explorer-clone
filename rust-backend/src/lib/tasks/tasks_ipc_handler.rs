use std::sync::Arc;

use serde_json::{json, Error, Value};

use crate::{
    globals::Globals,
    user_events::{UserEvent, WebviewEvent},
    user_events_handler::TaskIPCMsg,
    utils::{construct_js_event, get_field_as_string},
};

pub fn tasks_ipc_handler(window_key: &str, body: &Value, globals: Arc<Globals>) {
    // let action = get_field_as_string(body, "action");
    println!(
        "tasks_ipc_handler: window_key: {}, body: {:#?}",
        window_key, body
    );
    let ipc_msg: Result<TaskIPCMsg, Error> = serde_json::from_value(body.clone());

    if ipc_msg.is_err() {
        println!("Error parsing TaskIPCMsg: {}", ipc_msg.err().unwrap());
        return;
    }

    let ipc_msg = ipc_msg.unwrap();

    //Start
    if ipc_msg.action == "start" {
        tokio::task::spawn({
            let task_name = ipc_msg.task_name.clone();
            let id = ipc_msg.id.clone();
            let task_id = ipc_msg.task_id.clone();

            let args = ipc_msg.args; //body.get("args").cloned();
            let window_key = window_key.to_string();
            let globals = globals.clone();

            async move {
                let event_name = { globals.tasks_manager.lock().await }.start_task(
                    &task_name,
                    &task_id,
                    &window_key,
                    &Some(args),
                    globals.clone(),
                );

                let (error, event_name) = if event_name.is_ok() {
                    ("".into(), event_name.unwrap())
                } else {
                    (event_name.err().unwrap_or(String::new()), "".into())
                    //TODO: Maybe also fire frontend task error eventlistener
                };

                let response = json!({"id":id, "error": error, "data": event_name  });
                let js_event = construct_js_event("ipc-response", &response);

                let _ = globals.event_loop_proxy.send_event(UserEvent::WebviewEvent(
                    window_key.to_string(),
                    WebviewEvent::EvaluateScript(js_event),
                ));
            }
        });
        return;
    }

    //Msg
    if ipc_msg.action == "task_msg" {
        tokio::task::spawn({
            let event_name = ipc_msg.event_name.clone();
            let args = ipc_msg.args.clone(); //body.get("args").cloned();
            let globals = globals.clone();
            let window_key = window_key.to_string();

            async move {
                //let def = json!({});
                //   let args = args;
                let tasks_manager = { globals.tasks_manager.lock().await };

                let sender = tasks_manager.send_msg(&event_name, &args);
                if let Some(s) = sender {
                    s.await;
                } else {
                    let js_event = construct_js_event(
                        &format!("{}_error", event_name),
                        &json!({"error": "Event not found"}),
                    );

                    println!("Event not found: {event_name} {js_event}");

                    let _ = globals.event_loop_proxy.send_event(UserEvent::WebviewEvent(
                        window_key.to_string(),
                        WebviewEvent::EvaluateScript(js_event),
                    ));
                }
            }
        });
        return;
    }

    //Force kill task
    //Reason: If a task is stuck, it will not be able to receive messages from the frontend, so we need to force kill it
    if ipc_msg.action == "force_kill" {
        println!("Force kill task with event_name: {}", ipc_msg.event_name);
        tokio::task::spawn({
            let event_name = ipc_msg.event_name.clone();
            let globals = globals.clone();
            let window_key = window_key.to_string();

            async move {
                let mut tasks_manager = { globals.tasks_manager.lock().await };
                let res = tasks_manager.end_task(&event_name);
                if let Err(e) = res {
                    let js_event =
                        construct_js_event(&format!("{}_error", event_name), &json!({"error": e}));

                    let _ = globals.event_loop_proxy.send_event(UserEvent::WebviewEvent(
                        window_key.to_string(),
                        WebviewEvent::EvaluateScript(js_event),
                    ));
                }

                println!("Task killed: {event_name}");
            }
        });
        return;
    }

    //KillAll!
    if ipc_msg.action == "kill_all" {
        tokio::task::spawn({
            let window_key = window_key.to_string();
            async move {
                println!("kill all tasks with window_key: {window_key}");
                let mut tasks_manager = { globals.tasks_manager.lock().await };
                tasks_manager.end_window_tasks(&window_key)
            }
        });
        return;
    }
}
