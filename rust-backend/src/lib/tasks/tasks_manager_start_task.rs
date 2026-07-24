use std::{collections::VecDeque, sync::Arc};

use serde_json::{json, Value};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
};

use crate::{
    globals::Globals,
    task_args::{TaskArgs, TaskHandle, TaskMsg},
    tasks_manager::{ActiveTask, TaskManager},
    user_events::{UserEvent, WebviewEvent},
    utils::construct_js_event,
};

impl TaskManager {
    pub async fn use_task_handle(
        handle: JoinHandle<String>,
        globals: Arc<Globals>,
        window_key: &str,
        event_name: &str,
    ) {
        match handle.await {
            Ok(m) => {
                let js_event = construct_js_event(&format!("{}_exit", event_name), &json!(m));

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
                    .remove(&event_name.to_string());

                println!("{event_name} exited...");
            }

            Err(e) => {
                if e.is_panic() {
                    println!("{event_name} panicked...");
                    //Remove task from active tasks
                    globals
                        .tasks_manager
                        .lock()
                        .await
                        .active_tasks
                        .remove(&event_name.to_string());
                }

                let error_js_event =
                    construct_js_event(&format!("{}_error", event_name), &json!(e.to_string()));
                let exit_js_event =
                    construct_js_event(&format!("{}_exit", event_name), &json!(e.to_string()));

                let js_event = if e.is_cancelled() {
                    //Exit
                    exit_js_event
                } else {
                    //Combine error and exit events
                    format!("{}{}", error_js_event, exit_js_event)
                };

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
                    .remove(&event_name.to_string());

                println!("{event_name} exited with error");
            }
        }
        println!("Task handle spawn ended for {event_name}...");
    }

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
        let abort_handle = handle.abort_handle();
        tokio::task::spawn({
            let event_name = event_name.clone();
            let window_key = window_key.to_string();

            async move {
                TaskManager::use_task_handle(handle, globals, &window_key, &event_name).await;
            }
        });

        self.active_tasks.insert(
            event_name.clone(),
            ActiveTask {
                abort_handle,
                sender: tx,
                window_key: window_key.to_string(),
            },
        );

        println!("{event_name} started...");
        Ok(String::from(event_name))
    }
}
