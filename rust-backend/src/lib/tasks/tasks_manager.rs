use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use serde_json::{json, Value};
use tokio::sync::{
    mpsc::Sender,
    mpsc::{channel, Receiver},
};
use uuid::Uuid;

use crate::{
    globals::Globals,
    task_args::{MsgSender, TaskArgs, TaskMsg},
    utils::get_field_as_string,
};

pub type TaskBoxFuture = Pin<Box<dyn Future<Output = Result<String, String>> + Send + Sync>>;
pub type TaskFnType = Arc<Box<dyn Fn(TaskArgs) -> TaskBoxFuture + Send + Sync>>;

pub struct Task {
    pub name: String,
    pub function: TaskFnType,
}

pub struct ActiveTask {
    //  listeners:i32,
    // thread_id: JoinHandle<F::Output>
}

pub struct TaskManager {
    /* name, task */
    pub tasks: HashMap<String, TaskFnType>,
    /* task id, (listeners) */
    //  pub active_tasks: HashMap<String, i32>,
    /* task id, sender */
    pub task_channels: HashMap<String, Sender<TaskMsg>>,
    pub globals: Arc<Globals>,
}

impl TaskManager {
    pub fn register_task(&mut self, task: Task) {
        self.tasks.insert(task.name, task.function);
    }

    pub fn start_task(
        &mut self,
        task_name: &str,
        start_msg: Option<&Value>,
    ) -> Result<String, String> {
        let start_msg = start_msg.cloned();
        //Find task
        let task = self.tasks.get(task_name);
        if task.is_none() {
            return Err("Task not found".into());
        }

        let task_id = Uuid::new_v4().to_string();
        let event_name = format!("{task_name}_{task_id}");

        //Setup communication channel
        let (tx, rx): (Sender<TaskMsg>, Receiver<TaskMsg>) = channel(32);

        let globals = self.globals.clone();

        //Task recieve end
        let task_args = TaskArgs {
            listener_buffer: start_msg,
            manager_buffer: None,
            reciever: rx,
            globals: globals.clone(),
            event_name: event_name.clone(),
        };

        //Task manager send end
        self.task_channels.insert(event_name.clone(), tx);

        //Start task
        let task = task.unwrap().clone();
        let _handle = tokio::task::spawn(async move {
            let exit_msg = task(task_args).await.unwrap_or("".into());
            println!("Exit msg {exit_msg}");
            //Broadcast
        });

        println!("{event_name} started...");
        Ok(String::from(event_name))
    }

    pub fn send_msg(
        &self,
        event_name: &str,
        msg: &Value,
    ) -> Option<Pin<Box<dyn Future<Output = ()> + Send + Sync + '_>>> {
        let channel = self.task_channels.get(event_name);

        let msg = msg.clone();
        let sender = if get_field_as_string(&msg, "sender") == "manager" {
            MsgSender::Manager
        } else {
            MsgSender::Listener
        };

        if let Some(c) = channel {
            let fx = c.send(TaskMsg {
                data: json!({"data":get_field_as_string(&msg,"data")}),
                sender,
            });

            let f = async move {
                let _ = fx.await;
            };

            return Some(Box::pin(f));
        }
        None
    }
}
