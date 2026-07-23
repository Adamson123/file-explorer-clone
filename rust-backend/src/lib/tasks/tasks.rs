use std::{sync::Arc, time::Duration};

use serde_json::{json, Value};
use tokio::time::sleep;

use crate::{
    commands::get_dir_c,
    task_args::{TaskArgs, TaskHandle},
    tasks_manager::{Task, TaskBoxFuture},
    utils::get_field_as_string,
};

// macro_rules! repeat {
//     ($count:expr, $args:expr, |$var:ident| | $body:block) => {
//         for $var in 0..$count {
//             $body
//         }
//     };
// }

macro_rules! repeat {
    (
        $count:expr,
        //$delay:expr,
        |$var:ident| $body:block
    ) => {
        // for $var in 0..$count {
        //     $body

        //     //   sleep(Duration::from_secs($delay)).await;
        // }
        let mut $var = 0;
        while $var < $count {
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

    repeat!(100, |i| {
        match a.get_task_state() {
            TaskHandle::Run => {}
            TaskHandle::Pause => {
                sleep(Duration::from_millis(1)).await;
                continue;
            }
            TaskHandle::Cancel => break,
        }

        let x = a.recv_listener_msg().unwrap_or(def.clone());

        // println!(
        //     "Recieved: {}",
        //     a.listener_buffer.clone().unwrap_or(def.clone())
        // );
        // if x.as_object().unwrap().len() > 0 {
        //     println!("Recieved: {}", x);
        //     a.send_msg(&x).await;
        // }

        // println!("Iteration: {i}");
        // println!("{x}");

        match &x {
            Value::Object(_x) => {
                if x.as_object().unwrap().len() > 0 {
                    // println!("Recieved: {x}");
                    // println!("Iteration: {i}");
                    // a.send_msg(&json!({"your_msg":x,"rust_msg":"back from rust"}))
                    //     .await;

                    println!("Recieved: {}", x);
                    let path = get_field_as_string(&x, "path");
                    if !path.is_empty() {
                        println!("current_path saved");
                        current_path = x.clone();
                    }
                }
            }
            _ => {
                println!("Recieved: {}", x);
            }
        }

        if current_path.as_object().unwrap().len() > 0 {
            let dir_c = get_dir_c(&current_path, a.globals.clone())
                .await
                .unwrap_or(String::new());
            //  println!("😂😂😂{dir_c}");
            a.send_msg(&json!(dir_c)).await;
        }

        sleep(Duration::from_secs(4)).await;
    });

    println!("Monitor done");

    //format!("{} is done...", a.event_name)
    Ok(String::from(
        json!({"msg":format!("{} is done...", a.event_name)}).to_string(),
    ))
});
