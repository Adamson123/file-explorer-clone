use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};
use tao::event_loop::EventLoopProxy;

use crate::{
    globals::Globals,
    user_events::{UserEvent, WebviewEvent},
    utils::construct_js_event,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandIPCMsg {
    pub cmd: String,
    pub args: Value,
    pub request_id: String,
}

pub async fn send_ipc_response(
    proxy: EventLoopProxy<UserEvent>,
    request_id: &str,
    window_key: &str,
    res: Result<String, String>,
) {
    let def = String::from(r#""#);
    let data = res.as_ref().unwrap_or(&def);
    let error = res.as_ref().err().unwrap_or(&def);

    let json_response = serde_json::json!({
        "data":data,
        "error":error,
        "id":request_id
    });

    let js_event = construct_js_event("ipc-response", &json_response);

    let _ = proxy.send_event(UserEvent::WebviewEvent(
        window_key.into(),
        WebviewEvent::EvaluateScript(js_event),
    ));
}

pub async fn commands_ipc_handler(window_key: &str, body: &Value, globals: Arc<Globals>) {
    let ipc_msg: Result<CommandIPCMsg, Error> = serde_json::from_value(body.clone());
    if ipc_msg.is_err() {
        println!("Error parsing CommandIPCMsg: {}", ipc_msg.err().unwrap());
        return;
    }
    let ipc_msg = ipc_msg.unwrap();

    let mut args_map = ipc_msg
        .args
        .as_object()
        .cloned()
        .unwrap_or_else(|| serde_json::Map::new());

    // 2. Insert the new key into the Map
    args_map.insert("window_key".to_string(), json!(window_key));

    // 3. Convert the Map into a Value (Object variant)
    let args: Value = Value::Object(args_map);

    let cmd = ipc_msg.cmd.clone();
    let request_id: String = ipc_msg.request_id.clone();

    // println!("Body: {}", body);
    // println!("args: {}, cmd: {}, id: {}", args, cmd, id);

    let command = {
        let commands_register = globals.commands_register.lock().await;
        commands_register.invoke_command(&cmd, &args, globals.clone())
    };

    let res = match command {
        Ok(f) => f.await,
        Err(e) => Err(e),
    };

    send_ipc_response(
        globals.event_loop_proxy.clone(),
        &request_id,
        window_key,
        res,
    )
    .await;
}
