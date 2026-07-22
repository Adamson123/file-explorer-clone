use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc, time::Duration};

use serde_json::{json, Value};
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time::sleep,
};
use uuid::Uuid;

use crate::{
    globals::Globals,
    task_args::{MsgSender, TaskArgs, TaskHandle, TaskMsg},
    user_events::{UserEvent, WebviewEvent},
    utils::{construct_js_event, get_field_as_string},
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
    // pub globals: Arc<Globals>,
}

impl TaskManager {
    pub fn register_task(&mut self, task: Task) {
        self.tasks.insert(task.name, task.function);
    }

    pub fn start_task(
        &mut self,
        task_name: &str,
        window_key: &str,
        start_msg: &Option<Value>,
        globals: Arc<Globals>,
    ) -> Result<String, String> {
        let mut start_msg = start_msg.clone();
        if start_msg.is_some() {
            let data = get_field_as_string(&start_msg.clone().unwrap(), "data");
            start_msg = Some(json!({"data":data}));
        }

        //Find task
        let task = self.tasks.get(task_name);
        if task.is_none() {
            return Err("Task not found".into());
        }

        let task_id = Uuid::new_v4().to_string();
        let event_name = format!("{task_name}_{task_id}");

        //Setup communication channel
        let (tx, rx): (Sender<TaskMsg>, Receiver<TaskMsg>) = channel(32);

        //let globals = globals.clone();

        //Task recieve end

        let task_args = TaskArgs {
            listener_buffer: vec![start_msg.unwrap_or(json!({}))], //start_msg.clone(),
            manager_buffer: Vec::new(),

            listener_last_read: None, //start_msg,
            manager_last_read: None,

            reciever: rx,
            globals: globals.clone(),
            event_name: event_name.clone(),
            window_key: window_key.to_string(),
            task_handle: TaskHandle::Run,
        };

        //Task manager send end
        // let tx_clone = tx.clone();
        self.task_channels.insert(event_name.clone(), tx);

        //Start task
        let task = task.unwrap().clone();
        let _handle = tokio::task::spawn({
            let window_key = window_key.to_string();
            let globals = globals.clone();
            let event_name = event_name.clone();

            async move {
                sleep(Duration::from_secs_f64(1.5)).await;
                let exit_msg = task(task_args).await.unwrap_or("".into());

                let js_event =
                    construct_js_event(&format!("{}_exit", event_name), &json!(exit_msg));

                let _ = globals.tasks_event_loop_proxy.lock().await.send_event(
                    UserEvent::WebviewEvent(
                        window_key.to_string(),
                        WebviewEvent::EvaluateScript(js_event),
                    ),
                );
            }
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
        // println!(
        //     "Original data: {}",
        //     msg.get("data").unwrap_or(&json!({})).clone()
        // );

        let sender = if get_field_as_string(&msg, "sender") == "manager" {
            MsgSender::Manager
        } else {
            MsgSender::Listener
        };

        if let Some(c) = channel {
            let fx = c.send(TaskMsg {
                data: msg.get("data").unwrap_or(&json!({})).clone(),
                sender,
            });

            // println!(
            //     "Original data again: {}",
            //     msg.get("data").unwrap_or(&json!({})).clone()
            // );

            let f = async move {
                let _ = fx.await;
            };

            return Some(Box::pin(f));
        }
        None
    }
}
