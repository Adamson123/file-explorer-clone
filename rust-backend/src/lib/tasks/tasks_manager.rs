use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use serde_json::{json, Value};
use tokio::{sync::mpsc::Sender, task::AbortHandle};

use crate::{
    task_args::{MsgSender, TaskArgs, TaskMsg},
    utils::get_field_as_string,
};

pub type TaskBoxFuture = Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;
pub type TaskFnType = Arc<Box<dyn Fn(TaskArgs) -> TaskBoxFuture + Send + Sync>>;

pub struct Task {
    pub name: String,
    pub function: TaskFnType,
}

pub struct ActiveTask {
    pub abort_handle: AbortHandle,
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

    pub fn end_task(&mut self, event_name: &str) -> Result<String, String> {
        if let Some(active_task) = self.active_tasks.remove(event_name) {
            active_task.abort_handle.abort();
            println!("Ended task with key : {event_name}");
            return Ok(String::from("Task ended successfully"));
        }

        Err(String::from("Task not found"))
    }

    pub fn end_window_tasks(&mut self, window_key: &str) {
        if !self.active_tasks.is_empty() {
            self.active_tasks.retain(|k, value| {
                if value.window_key != window_key {
                    return true;
                } else {
                    value.abort_handle.abort();
                    println!("Ended task with key : {k}");
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
                    value.abort_handle.abort();
                    println!("Ended task with key : {k}");
                    return false;
                }
            });
        }
    }
}
