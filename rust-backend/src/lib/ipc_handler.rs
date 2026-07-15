use std::sync::Arc;

use serde_json::{json, Value};
use tao::event_loop::EventLoopProxy;
use tokio::sync::Mutex;
use wry::http::Request;

use crate::{
    globals::Globals,
    user_events::{UserEvent, WebviewEvent},
    utils::get_field_as_string,
};

pub fn send_ipc_response(proxy: &EventLoopProxy<UserEvent>, id: &str, res: Result<String, String>) {
    let def = String::from(r#""#);
    let data = res.as_ref().unwrap_or(&def);
    let error = res.as_ref().err().unwrap_or(&def);

    let js = format!(
        r#"
        document.dispatchEvent(
    new CustomEvent("ipc-response", {{
        detail: {{
         data:`{}`,
         error:`{}`,
         id:"{}"
      }},

    }}),
  );
        "#,
        data, error, id
    );

    //println!("{}", js);
    //let _ = webview.evaluate_script(&js);
    // let _ = webview.evaluate_script(&js);
    let _ = proxy.send_event(UserEvent::WebviewEvent(WebviewEvent::EvaluateScript(js)));
}

pub async fn handle_ipc_msg(
    msg: &Request<String>,
    //   commands_reg: &CommandsRegistry,
    globals: Arc<Mutex<Globals>>,
) {
    let def = json!({});
    let body: Value = serde_json::from_str(msg.body()).unwrap_or((&def).to_owned());

    let args = body.get("args").unwrap_or(&def);
    let cmd = get_field_as_string(&body, "cmd");
    let id = get_field_as_string(&body, "id");

    // println!("Body: {}", body);
    // println!("args: {}, cmd: {}, id: {}", args, cmd, id);

    let globals = globals.lock().await;
    let commands_reg = &globals.commands_reg;
    let res = commands_reg.invoke_command(&cmd, args, &globals).await;

    send_ipc_response(&globals.event_loop_proxy, &id, res);
}
