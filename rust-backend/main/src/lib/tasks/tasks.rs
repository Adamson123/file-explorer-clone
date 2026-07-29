use std::time::Duration;

use rusty_bridge::{repeat, task};
use serde_json::json;
use tokio::time::sleep;

use crate::{commands::get_dir_c, states::AppState};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct Dir {
    pub path: String,
}

task!(monitor_dir, |a| {
    println!("Starting Monitor...");

    let mut current_dir = Dir {
        path: String::new(),
    };

    repeat!(a, |_i| {
        let msg = a.recv_listener_msg().await;

        println!("Running!!!");
        if msg.is_some() {
            let dir: Option<Dir> = serde_json::from_value(msg.unwrap()).unwrap_or(None);
            if dir.is_some() {
                current_dir.path = dir.unwrap().path;
            }
        }

        if current_dir.path.is_empty() {
            sleep(Duration::from_secs(2)).await;
            //a.set_state(&TaskHandle::Pause);
            continue;
        }

        let dir_c = get_dir_c(
            &json!({"path": current_dir.path.clone()}),
            a.globals.clone(),
        )
        .await;

        match dir_c {
            Ok(c) => {
                let last_dir_info = a.globals.states_manager.get_state::<AppState>();
                if let Some(state) = last_dir_info {
                    let mut info = state.information.lock().await;
                    info.path = current_dir.path.clone();
                    info.contents = json!(c);
                };

                a.send_msg(&json!(c)).await;
            }
            Err(e) => {
                a.send_err_msg(&json!(e)).await;
                // current_dir.path = String::new();
            }
        };

        sleep(Duration::from_secs(2)).await;
    });

    println!("Monitor done");
    Ok(String::from(
        json!({"msg":format!("{} is done...", a.event_name)}).to_string(),
    ))
});
