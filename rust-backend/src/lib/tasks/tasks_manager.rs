use std::{
    collections::{HashMap, VecDeque},
    future::Future,
    pin::Pin,
    sync::Arc,
};

use serde_json::{json, Value};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
};

use crate::{
    globals::Globals,
    task_args::{MsgSender, TaskArgs, TaskHandle, TaskMsg},
    user_events::{UserEvent, WebviewEvent},
    utils::{construct_js_event, get_field_as_string},
};

pub type TaskBoxFuture = Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;
pub type TaskFnType = Arc<Box<dyn Fn(TaskArgs) -> TaskBoxFuture + Send + Sync>>;

pub struct Task {
    pub name: String,
    pub function: TaskFnType,
}

pub struct ActiveTask {
    pub handle: JoinHandle<()>,
    pub sender: Sender<TaskMsg>,
    pub window_key: String,
}

impl ActiveTask {
    pub fn send_msg(&self, msg: &Value) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let msg = msg.clone();

        let sender = if get_field_as_string(&msg, "sender") == "manager" {
            MsgSender::Manager
        } else {
            MsgSender::Listener
        };

        let f = self.sender.send(TaskMsg {
            data: msg.get("data").unwrap_or(&json!({})).clone(),
            sender,
        });
        Box::pin(async move {
            let _ = f.await;
        })
    }
}

pub struct TaskManager {
    /* name, task */
    pub tasks: HashMap<String, TaskFnType>,
    /* event_name, active_task */
    pub active_tasks: HashMap<String, ActiveTask>,
    // pub globals: Arc<Globals>,
}
//TODO: Tasks of closed windows should be shutdown
impl TaskManager {
    pub fn register_task(&mut self, task: Task) {
        self.tasks.insert(task.name, task.function);
    }

    pub fn start_task(
        &mut self,
        task_name: &str,
        task_id: &str,
        window_key: &str,
        start_msg: &Option<Value>,
        globals: Arc<Globals>,
    ) -> Result<String, String> {
        let mut start_msg = start_msg.clone();

        if start_msg.is_some() {
            let data = start_msg.unwrap();
            let d = json!({});
            let data = data.get("data").unwrap_or(&d).clone();
            start_msg = Some(data);
        }

        let task = self.tasks.get(task_name);
        //Find task
        let event_name = format!("{task_name}_{task_id}");
        //Setup communication channel
        let (tx, rx): (Sender<TaskMsg>, Receiver<TaskMsg>) = channel(32);

        if task.is_none() {
            return Err("Task not found".into());
        }

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
        let handle = tokio::task::spawn({
            let window_key = window_key.to_string();
            let globals = globals.clone();
            let event_name = event_name.clone();
            let task_id = task_id.to_string();

            async move {
                //  sleep(Duration::from_secs_f64(1.5)).await;
                let exit_msg = task(task_args).await.unwrap_or("".into());

                let js_event =
                    construct_js_event(&format!("{}_exit", event_name), &json!(exit_msg));

                let _ = globals.event_loop_proxy.send_event(UserEvent::WebviewEvent(
                    window_key.to_string(),
                    WebviewEvent::EvaluateScript(js_event),
                ));

                globals
                    .tasks_manager
                    .lock()
                    .await
                    .active_tasks
                    .remove(&task_id);
            }
        });

        self.active_tasks.insert(
            event_name.clone(),
            ActiveTask {
                handle,
                sender: tx,
                window_key: window_key.to_string(),
            },
        );

        println!("{event_name} started...");
        Ok(String::from(event_name))
    }

    pub fn send_msg(
        &self,
        event_name: &str,
        msg: &Value,
    ) -> Option<Pin<Box<dyn Future<Output = ()> + Send + '_>>> {
        let active_task = self.active_tasks.get(event_name);

        if active_task.is_some() {
            let f = active_task.unwrap().send_msg(msg);
            Some(Box::pin(f))
        } else {
            None
        }
    }

    pub fn end_window_tasks(&mut self, window_key: &str) {
        if !self.active_tasks.is_empty() {
            self.active_tasks.retain(|k, value| {
                if value.window_key != window_key {
                    return true;
                } else {
                    println!("Ended task with key : {k}",);
                    value.handle.abort();
                    return false;
                }
            });
        }
    }

    pub fn end_multiple_window_tasks(&mut self, windows_key: &Vec<String>) {
        if !self.active_tasks.is_empty() {
            self.active_tasks.retain(|k, value| {
                if !windows_key.contains(&value.window_key) {
                    return true;
                } else {
                    println!("Ended task with key : {k}",);
                    value.handle.abort();
                    return false;
                }
            });
        }
    }
}
