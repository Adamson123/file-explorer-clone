use std::time::Duration;

use crate::{commands::get_dir_c, repeat_while, task, utils::get_field_as_string};
use serde_json::{json, Value};
use tokio::time::sleep;

task!(monitor_dir, |a| {
    println!("Starting Monitor...");

    let def = json!({});
    let mut current_path = json!({});

    let mut j = 0;

    repeat_while!(j < 100, a, |_i| {
        let x = a.recv_listener_msg().await;

        if x.is_some() {
            println!("Got this: {}", x.clone().unwrap());
            a.send_msg(&json!({"from":"rust","msg":x.clone()})).await;
        }

        let x = x.unwrap_or(def.clone());
        match &x {
            Value::Object(_x) => {
                if x.as_object().unwrap().len() > 0 {
                    // println!("Recieved: {}", x);
                    let path = get_field_as_string(&x, "path");
                    if !path.is_empty() {
                        println!("current_path saved");
                        current_path = x.clone();
                    }
                }
            }
            _ => {
                //println!("Recieved: {}", x);
            }
        }

        if current_path.as_object().unwrap().len() > 0 {
            let dir_c = get_dir_c(&current_path, a.globals.clone())
                .await
                .unwrap_or(String::new());
            //  println!("😂😂😂{dir_c}");
            a.send_msg(&json!(dir_c)).await;
        }

        sleep(Duration::from_secs(2)).await;
        j += 1;
    });

    println!("Monitor done");

    //format!("{} is done...", a.event_name)
    Ok(String::from(
        json!({"msg":format!("{} is done...", a.event_name)}).to_string(),
    ))
});
