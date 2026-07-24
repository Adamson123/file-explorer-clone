use std::{sync::Arc, time::Duration};

use serde_json::{json, Value};
use tokio::time::sleep;

use crate::{
    commands::get_dir_c,
    task_args::{TaskArgs, TaskHandle},
    tasks_manager::{Task, TaskBoxFuture},
    utils::get_field_as_string,
};

macro_rules! repeat_while {
    (
        $condition:expr,
        $a:expr,
        |$var:ident| $body:block
    ) => {

        let mut $var = 0;

        while $condition {
            match $a.get_task_state().await {
            TaskHandle::Run => {}
            TaskHandle::Pause => {
              //  sleep(Duration::from_millis(1)).await;
                continue;
            }
            TaskHandle::Cancel => break,
        }

            $body

            $var += 1;
        }
    };
}

macro_rules! _repeat_for {
    (
        $count:expr,
        $a:expr,
        |$var:ident| $body:block
    ) => {
        let mut $var = 0;
        while $var < $count {
            match $a.get_task_state() {
            TaskHandle::Run => {}
            TaskHandle::Pause => {
                sleep(Duration::from_millis(1)).await;
                continue;
            }
            TaskHandle::Cancel => break,
        }

            $body

            $var += 1;
        }
    };
}

#[macro_export]
macro_rules! task {
    (
        $name:ident,
        |$args:ident|
        $body:block
    ) => {
        pub fn $name() -> Task {
            fn fnt(mut $args: TaskArgs) -> TaskBoxFuture {
                Box::pin(async move {
                    let res = { $body };
                    res
                })
            }

            Task {
                name: stringify!($name).to_string(),
                function: Arc::new(Box::new(fnt)),
            }
        }
    };
}

task!(monitor_dir, |a| {
    println!("Started Monitor");

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
