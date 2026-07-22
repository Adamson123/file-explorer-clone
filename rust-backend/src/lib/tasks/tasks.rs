use std::sync::Arc;

use serde_json::json;
use tokio::task::yield_now;

use crate::{
    task_args::TaskArgs,
    tasks_manager::{Task, TaskBoxFuture},
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
        $args:expr,
        |$var:ident| $body:block
    ) => {
        for $var in 0..$count {
            $body
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

    repeat!(1000000, a, |i| {
        let x = a.recv_listener_msg().unwrap_or(def.clone());
        if x.as_object().unwrap().len() > 0 {
            println!("Recieved: {x}");
            println!("Iteration: {i}");
        }
        a.send_msg(&x).await;

        yield_now().await;
        //  println!("Args: {:?}", a);
    });

    println!("Monitor done");

    Ok(String::from("Done ooooo"))
});
