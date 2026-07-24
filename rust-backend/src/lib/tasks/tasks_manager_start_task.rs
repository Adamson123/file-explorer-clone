use std::{collections::VecDeque, sync::Arc};

use serde_json::{json, Value};
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex,
};

use crate::{
    globals::Globals,
    task_args::{TaskArgs, TaskHandle, TaskMsg},
    tasks_manager::{ActiveTask, TaskManager},
    user_events::{UserEvent, WebviewEvent},
    utils::construct_js_event,
};

impl TaskManager {
    pub fn start_task(
        &mut self,
        task_name: &str,
        task_id: &str,
        window_key: &str,
        start_msg: &Option<Value>,
        globals: Arc<Globals>,
    ) -> Result<String, String> {
        let event_name = format!("{task_name}_{task_id}");

        if self.active_tasks.contains_key(&event_name) {
            return Err(format!("{event_name} is already running..."));
        }

        let mut start_msg = start_msg.clone();
        if start_msg.is_some() {
            let data = start_msg.unwrap();
            let d = json!({});
            let data = data.get("data").unwrap_or(&d).clone();
            start_msg = Some(data);
        }

        //Find task
        let task = self.tasks.get(task_name);
        if task.is_none() {
            return Err("Task not found".into());
        }
        //Setup communication channel
        let (tx, rx): (Sender<TaskMsg>, Receiver<TaskMsg>) = channel(32);

        //Task recieve end
        let task_args = TaskArgs {
            listener_buffer: VecDeque::from([start_msg.unwrap_or(json!({}))]), //start_msg.clone(),
            manager_buffer: VecDeque::new(),

            listener_last_read: None, //start_msg,
            manager_last_read: None,

            reciever: rx,
            event_name: event_name.clone(),
            window_key: window_key.to_string(),
            task_handle: TaskHandle::Run,

            globals: globals.clone(),
        };

        //Start task
        let task = task.unwrap().clone();
        let handle = tokio::task::spawn(async move { task(task_args).await.unwrap_or("".into()) });

        let handle = Arc::new(Mutex::new(Some(handle)));

        tokio::task::spawn({
            println!("Task handle spawned for {event_name}...");
            let window_key = window_key.to_string();
            let globals = globals.clone();
            let event_name = event_name.clone();
            let handle = handle.clone();

            async move {
                let handle = {
                    let mut guard = handle.lock().await;
                    guard.take()
                };

                if handle.is_none() {
                    println!("Task handle is none for {event_name}...");
                    return;
                }

                match handle.unwrap().await {
                    Ok(m) => {
                        let js_event =
                            construct_js_event(&format!("{}_exit", event_name), &json!(m));

                        let _ = globals.event_loop_proxy.send_event(UserEvent::WebviewEvent(
                            window_key.to_string(),
                            WebviewEvent::EvaluateScript(js_event),
                        ));

                        //Remove task from active tasks
                        globals
                            .tasks_manager
                            .lock()
                            .await
                            .active_tasks
                            .remove(&event_name);

                        println!("{event_name} exited...");
                    }
                    Err(e) => {
                        let js_event = construct_js_event(
                            &format!("{}_error", event_name),
                            &json!(e.to_string()),
                        );
                        let exit_js_event = construct_js_event(
                            &format!("{}_exit", event_name),
                            &json!(e.to_string()),
                        );

                        let combine = format!("{js_event};{exit_js_event}");

                        let _ = globals.event_loop_proxy.send_event(UserEvent::WebviewEvent(
                            window_key.to_string(),
                            WebviewEvent::EvaluateScript(combine),
                        ));

                        //Remove task from active tasks
                        globals
                            .tasks_manager
                            .lock()
                            .await
                            .active_tasks
                            .remove(&event_name);

                        println!("{event_name} exited with error");
                    }
                }
                println!("Task handle spawn ended for {event_name}...");
            }
        });

        self.active_tasks.insert(
            event_name.clone(),
            ActiveTask {
                handle: handle,
                sender: tx,
                window_key: window_key.to_string(),
            },
        );

        println!("{event_name} started...");
        Ok(String::from(event_name))
    }
}
