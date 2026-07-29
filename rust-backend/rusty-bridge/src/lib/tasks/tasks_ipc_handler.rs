use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};

use crate::{
    globals::Globals,
    user_events::{UserEvent, WebviewEvent},
    utils::construct_js_event,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TaskAction {
    Start,
    TaskMsg,
    ForceKill,
    KillAll,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskIPCMsg {
    pub task_name: String,
    pub task_id: String,
    pub event_name: String,
    pub args: Value,
    pub request_id: String,
    pub action: TaskAction,
}

pub fn tasks_ipc_handler(window_key: &str, body: &Value, globals: Arc<Globals>) {
    let ipc_msg: Result<TaskIPCMsg, Error> = serde_json::from_value(body.clone());
    if ipc_msg.is_err() {
        println!("Error parsing TaskIPCMsg: {}", ipc_msg.err().unwrap());
        return;
    }

    let ipc_msg = ipc_msg.unwrap();
    match ipc_msg.action {
        TaskAction::Start => {
            tokio::task::spawn({
                let task_name = ipc_msg.task_name.clone();
                let request_id = ipc_msg.request_id.clone();
                let args = ipc_msg.args;
                let window_key = window_key.to_string();
                let globals = globals.clone();
                let event_name = ipc_msg.event_name.clone();

                async move {
                    let res = {
                        globals.tasks_manager.lock().await.start_task(
                            &task_name,
                            &event_name,
                            &window_key,
                            &Some(args),
                            globals.clone(),
                        )
                    };

                    let (error, res) = if res.is_ok() {
                        ("".into(), res.unwrap())
                    } else {
                        (res.err().unwrap_or(String::new()), "".into())
                    };

                    let response = json!({"request_id":request_id, "error": error, "data": res  });
                    let js_event = construct_js_event("ipc-response", &response);

                    let _ = globals.event_loop_proxy.send_event(UserEvent::WebviewEvent(
                        window_key.to_string(),
                        WebviewEvent::EvaluateScript(js_event),
                    ));
                }
            });
        }
        TaskAction::TaskMsg => {
            tokio::task::spawn({
                let event_name = ipc_msg.event_name.clone();
                let args = ipc_msg.args.clone();
                let globals = globals.clone();
                let window_key = window_key.to_string();
                let request_id = ipc_msg.request_id.clone();

                async move {
                    let sender = {
                        let tasks_manager = globals.tasks_manager.lock().await;
                        tasks_manager.send_msg(&event_name, &args)
                    };

                    if let Some(s) = sender {
                        s.await;
                    } else {
                        let response = json!({"request_id":request_id, "error": format!("Event not found: {}", event_name), "data": ""  });
                        let js_event = construct_js_event("ipc-response", &response);

                        let _ = globals.event_loop_proxy.send_event(UserEvent::WebviewEvent(
                            window_key.to_string(),
                            WebviewEvent::EvaluateScript(js_event),
                        ));
                    }
                }
            });
        }

        TaskAction::ForceKill => {
            tokio::task::spawn({
                let event_name = ipc_msg.event_name.clone();
                let globals = globals.clone();
                let window_key = window_key.to_string();
                let request_id = ipc_msg.request_id.clone();

                async move {
                    let res = {
                        let mut tasks_manager = globals.tasks_manager.lock().await;
                        tasks_manager.end_task(&event_name)
                    };

                    if let Err(e) = res {
                        let response = json!({"request_id":request_id, "error": format!("Error killing task: {}", e), "data": ""  });
                        let js_event = construct_js_event("ipc-response", &response);

                        let _ = globals.event_loop_proxy.send_event(UserEvent::WebviewEvent(
                            window_key.to_string(),
                            WebviewEvent::EvaluateScript(js_event),
                        ));
                    }

                    println!("Task killed: {event_name}");
                }
            });
        }
        TaskAction::KillAll => {
            tokio::task::spawn({
                let window_key = window_key.to_string();
                async move {
                    println!("kill all tasks with window_key: {window_key}");
                    {
                        let mut tasks_manager = globals.tasks_manager.lock().await;
                        tasks_manager.end_window_tasks(&window_key);
                    };
                }
            });
        }
    }
}
