use std::{collections::VecDeque, sync::Arc};

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

    pub listener_buffer: VecDeque<Value>,
    pub manager_buffer: VecDeque<Value>,

    pub listener_last_read: Option<Value>,
    pub manager_last_read: Option<Value>,

    pub event_name: String,
    pub window_key: String,

    pub task_handle: TaskHandle,

    pub globals: Arc<Globals>,
}

impl TaskArgs {
    pub async fn get_msg(&mut self) {
        //Reason for this is that if the task is paused, we want to wait for a message to come in, but if the task is running, we want to check if there is a message available and if not, continue on with the task.
        let msg: Option<TaskMsg> = match self.task_handle {
            TaskHandle::Pause => match self.reciever.recv().await {
                Some(m) => Some(m),
                None => None,
            },
            _ => match self.reciever.try_recv() {
                Ok(m) => Some(m),
                Err(_e) => None,
            },
        };

        if msg.is_none() {
            return;
        } else {
            //  println!("Try Rec got: {:#?}", msg.clone().unwrap());
            match msg.clone().unwrap().sender {
                MsgSender::Listener => {
                    self.listener_buffer.push_back(msg.clone().unwrap().data);
                }
                MsgSender::Manager => {
                    self.manager_buffer.push_back(msg.clone().unwrap().data);
                }
            }
        }
    }

    pub async fn recv_listener_msg(&mut self) -> Option<Value> {
        self.get_msg().await;

        if !self.listener_buffer.is_empty() {
            self.listener_last_read = Some(self.listener_buffer.pop_front().unwrap());
            return self.listener_last_read.clone();
        }

        None
    }

    async fn recv_manager_msg(&mut self) -> Option<Value> {
        self.get_msg().await;

        if !self.manager_buffer.is_empty() {
            // return Some(self.manager_buffer.remove(0));
            self.manager_last_read = Some(self.manager_buffer.pop_front().unwrap());
            return self.manager_last_read.clone();
        }
        None
    }

    pub async fn get_task_state(&mut self) -> TaskHandle {
        let msg = self.recv_manager_msg().await;

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

    pub fn set_state(&mut self, state: &TaskHandle) {
        self.task_handle = state.clone();
    }

    pub async fn send_msg(&self, msg: &Value) {
        let msg = msg.clone();
        let js_event = construct_js_event(&self.event_name, &msg);

        let _ = self
            .globals
            .event_loop_proxy
            .send_event(UserEvent::WebviewEvent(
                self.window_key.clone(),
                WebviewEvent::EvaluateScript(js_event),
            ));
    }

    pub async fn send_err_msg(&self, msg: &Value) {
        let msg = msg.clone();
        let js_event = construct_js_event(&format!("{}_error", self.event_name), &msg);

        let _ = self
            .globals
            .event_loop_proxy
            .send_event(UserEvent::WebviewEvent(
                self.window_key.clone(),
                WebviewEvent::EvaluateScript(js_event),
            ));
    }
}
