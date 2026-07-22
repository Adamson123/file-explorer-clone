use std::sync::Arc;

use serde_json::{json, Value};
use tokio::sync::mpsc::Receiver;

use crate::{
    globals::Globals,
    user_events::{UserEvent, WebviewEvent},
};

#[derive(Clone)]
pub enum MsgSender {
    Listener,
    Manager,
}

#[derive(Clone)]
pub struct TaskMsg {
    pub data: Value,
    pub sender: MsgSender,
}

pub struct TaskArgs {
    pub reciever: Receiver<TaskMsg>,
    pub listener_buffer: Option<Value>,
    pub manager_buffer: Option<Value>,
    pub event_name: String,
    pub globals: Arc<Globals>,
}

impl TaskArgs {
    pub fn get_msg(&mut self) -> Option<TaskMsg> {
        let msg: Option<TaskMsg> = match self.reciever.try_recv() {
            Ok(m) => Some(m),
            Err(_e) => None,
        };

        // if msg.is_some() {
        //     println!("Msg R: {}", msg.clone().unwrap().data);
        // } else {
        //      println!("No data")
        // }

        msg
    }

    pub fn recv_listener_msg(&mut self) -> Option<Value> {
        let msg = self.get_msg();

        if msg.is_none() {
            return None;
        }

        let msg = msg.unwrap();
        let msg = match msg.sender {
            MsgSender::Listener => Some(msg.data),
            MsgSender::Manager => None,
        };

        if msg.is_none() {
            return None;
        }

        self.listener_buffer = msg.clone();
        msg
    }

    pub fn recv_manager_msg(&mut self) -> Option<Value> {
        let msg = self.get_msg();

        if msg.is_none() {
            return None;
        }

        let msg = msg.unwrap();
        let msg = match msg.sender {
            MsgSender::Manager => Some(msg.data),
            MsgSender::Listener => None,
        };

        if msg.is_none() {
            return None;
        }

        self.manager_buffer = msg.clone();

        msg
    }

    pub async fn send_msg(&self, msg: &Value) {
        let msg = msg.clone();
        let _ = self
            .globals
            .event_loop_proxy
            .lock()
            .await
            .send_event(UserEvent::WebviewEvent(
                "*".into(),
                WebviewEvent::EvaluateScript(msg.to_string()),
            ));
    }
}
