use std::sync::Arc;

use serde_json::Value;
use tokio::sync::mpsc::Receiver;

use crate::{
    globals::Globals,
    user_events::{UserEvent, WebviewEvent},
    utils::{construct_js_event, get_field_as_string},
};

#[derive(Clone)]
pub enum TaskHandle {
    Run,
    Pause,
    Cancel,
}

#[derive(Clone, Debug)]
pub enum MsgSender {
    Listener,
    Manager,
}

#[derive(Clone, Debug)]
pub struct TaskMsg {
    pub data: Value,
    pub sender: MsgSender,
}

pub struct TaskArgs {
    pub reciever: Receiver<TaskMsg>,

    pub listener_buffer: Vec<Value>,
    pub manager_buffer: Vec<Value>,

    pub listener_last_read: Option<Value>,
    pub manager_last_read: Option<Value>,

    pub event_name: String,
    pub window_key: String,

    pub task_handle: TaskHandle,

    pub globals: Arc<Globals>,
}

impl TaskArgs {
    pub fn get_msg(&mut self) {
        let msg: Option<TaskMsg> = match self.reciever.try_recv() {
            Ok(m) => Some(m),
            Err(_e) => None,
        };

        if msg.is_none() {
            return;
        } else {
            //  println!("Try Rec got: {:#?}", msg.clone().unwrap());
            match msg.clone().unwrap().sender {
                MsgSender::Listener => {
                    self.listener_buffer.push(msg.clone().unwrap().data); //= Some(msg.clone().unwrap().data);
                }
                MsgSender::Manager => {
                    self.manager_buffer.push(msg.clone().unwrap().data); //= Some(msg.clone().unwrap().data);
                }
            }
        }
    }

    pub fn recv_listener_msg(&mut self) -> Option<Value> {
        self.get_msg();

        if !self.listener_buffer.is_empty() {
            self.listener_last_read = Some(self.listener_buffer.remove(0));
            return self.listener_last_read.clone();
        }

        None
    }

    fn recv_manager_msg(&mut self) -> Option<Value> {
        self.get_msg();

        if !self.manager_buffer.is_empty() {
            // return Some(self.manager_buffer.remove(0));
            self.manager_last_read = Some(self.manager_buffer.remove(0));
            return self.manager_last_read.clone();
        }

        None
    }

    pub fn get_task_state(&mut self) -> TaskHandle {
        let msg = self.recv_manager_msg();

        if msg.is_none() {
            return self.task_handle.clone();
        }

        let state = get_field_as_string(&msg.unwrap(), "state");
        let state = if state == "pause" {
            TaskHandle::Pause
        } else if state == "cancel" {
            TaskHandle::Cancel
        } else {
            TaskHandle::Run
        };

        self.task_handle = state.clone();
        state
    }

    pub async fn send_msg(&self, msg: &Value) {
        let msg = msg.clone();
        let js_event = construct_js_event(&self.event_name, &msg);

        let _ =
            self.globals
                .tasks_event_loop_proxy
                .lock()
                .await
                .send_event(UserEvent::WebviewEvent(
                    self.window_key.clone(),
                    WebviewEvent::EvaluateScript(js_event),
                ));
    }
}
