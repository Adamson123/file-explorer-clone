// use std::{sync::Arc, time::Duration};

// use serde_json::Value;
// use tokio::{task::yield_now, time::interval};

// use crate::{
//     commands_reg::{BoxFuture, Command, CommandFnType},
//     globals::Globals,
//     utils::get_field_as_string,
// };

// pub struct Task {
//     pub name: String,
//     pub function: CommandFnType,
// }

// #[macro_export]
// macro_rules! process_body_inner {
//     // ============================================================
//     // @repeat
//     // ============================================================
//     (
//         $args:ident,
//         $globals:ident,

//         @repeat ($count:expr)
//         |$var:ident|
//         $inner:block

//         $($rest:tt)*
//     ) => {{
//         let mut interval_var = interval(Duration::from_millis(1000));
//         for $var in 0..$count {
//             $inner
//             let w = get_field_as_string($args, "window_key");
//             println!("Windows key: {w} -> log from macro");
//             interval_var.tick().await;
//         }

//         $crate::process_body_inner!(
//             $args,
//             $globals,
//             $($rest)*
//         )
//     }};

//     // ============================================================
//     // Normal statement followed by more tokens
//     // ============================================================
//     (
//         $args:ident,
//         $globals:ident,

//         $statement:stmt;
//         $($rest:tt)*
//     ) => {{
//         $statement
//         $crate::process_body_inner!(
//             $args,
//             $globals,
//             $($rest)*
//         )
//     }};

//     // ============================================================
//     // Final expression
//     // ============================================================
//     (
//         $args:ident,
//         $globals:ident,

//         $expression:expr
//     ) => {
//         $expression
//     };
// }

// #[macro_export]
// macro_rules! process_body {
//     (
//         $args:ident,
//         $globals:ident,

//         $($body:tt)*
//     ) => {{
//         $crate::process_body_inner!(
//             $args,
//             $globals,
//             $($body)*
//         )
//     }};
// }

// #[macro_export]
// macro_rules! task {
//     (
//         $name:ident,
//         |$args:ident, $globals:ident| {
//             $($body:tt)*
//         }
//     ) => {
//         pub fn $name() -> Command {
//             fn fnt<'a>(
//                 $args: &'a Value,
//                 $globals: Arc<Globals>,
//             ) -> BoxFuture<'a> {
//                 Box::pin(async move {
//                     $crate::process_body!(
//                         $args,
//                         $globals,
//                         $($body)*
//                     )
//                 })
//             }

//             let name = stringify!($name).to_string();

//             Command {
//                 name,
//                 function: Arc::new(Box::new(fnt)),
//             }
//         }
//     };
// }

// task!(monitor_dir, |a, _g| {
//     let value = "Teasr";

//     @repeat(5) |i| {
//         println!("Iteration: {i}");
//         println!("Args: {:?}", a);
//     }

//     println!("Value: {value}");
//     Ok(String::from("From Monitor dir"))
// });
