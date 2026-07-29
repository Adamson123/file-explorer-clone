use std::{any::Any, fmt::Debug, sync::Arc};

use file_explorer_clone::{
    commands::{log_window_key, CustomEvent},
    tasks::monitor_dir,
};
use rusty_bridge::start::RustyBridgeBuilder;
use tao::event::Event;
use tokio::sync::Mutex;

// fn get_data<T: 'static>(data: Box<dyn Any + Send>) -> Option<T> {
//     let d = data.downcast::<T>();
//     match d {
//         Ok(o) => Some(*o),
//         Err(_e) => {
//             println!("Error downcasting");
//             return None;
//         }
//     }
// }

// impl Store {
//     fn get_data<T: 'static>(&mut self) -> Option<&mut T> {
//         let d = self.data.downcast_mut::<T>();
//         match d {
//             Some(o) => Some(o),
//             None => {
//                 println!("Error downcasting");
//                 return None;
//             }
//         }
//     }
// }

struct Store {
    data: Box<dyn Any + Send>,
}

impl Store {
    fn get_data<T: 'static + Clone>(&mut self) -> Option<T> {
        let d = self.data.downcast_ref::<T>();
        match d {
            Some(o) => Some(o.clone()),
            None => {
                println!("Error downcasting");
                return None;
            }
        }
    }

    fn new<T: Any + Send>(data: T) -> Self {
        Self {
            data: Box::new(data),
        }
    }

    fn get<T: Any>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }

    fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.data.downcast_mut::<T>()
    }

    fn get_mut_t<T: Any>(&mut self) -> Option<&mut T> {
        self.data.downcast_mut::<T>()
    }
}
#[derive(Debug)]
struct Info {
    pub name: String,
    pub age: i32,
}

struct AppState {
    counter: Mutex<Info>,
}

#[tokio::main]
async fn main() {
    // let mut store = Store {
    //     data: Box::new(AppState {
    //         counter: Arc::new(Mutex::new(Info {
    //             name: String::from("Adamson"),
    //             age: 20,
    //         })),
    //     }),
    // };

    // let data = store.get_data::<AppState>();
    // if data.is_some() {
    //     // println!("Down casted to: {:#?}", data.unwrap().counter.lock().await);
    //     let info = data.unwrap();
    //     let mut info = info.counter.lock().await;

    //     info.age = 1021110000;
    //     info.name = "Another Adam!!!".to_string();
    // }

    // let data = store.get_data::<AppState>();
    // if data.is_some() {
    //     println!(
    //         "After MODIFICATION Down casted to: {:#?}",
    //         data.unwrap().counter.lock().await
    //     );
    // }

    let mut store = Store {
        data: Box::new(AppState {
            counter: Mutex::new(Info {
                name: String::from("Adamson"),
                age: 20,
            }),
        }),
    };

    let data = store.get_mut::<AppState>();
    if data.is_some() {
        let info = data.unwrap();
        let mut info = info.counter.lock().await;

        info.age = 1021110000;
        info.name = "Another Adam!!!".to_string();
    }

    let data = store.get::<AppState>();

    if data.is_some() {
        let info = data.unwrap();
        let info = info.counter.lock().await;

        println!("After MODIFICATION Down casted to: {:#?}", info);
    }

    RustyBridgeBuilder::new()
        .url("http://localhost:5173")
        .register_commands(vec![log_window_key()])
        .register_tasks(vec![monitor_dir()])
        .handle_custom_event(|e, m| {
            if let Some(event) = e.downcast_ref::<CustomEvent>() {
                match *event {
                    CustomEvent::LogWindowKey => {
                        println!(
                            "I am logging the window keys from the custom event handler: {:#?}",
                            m.webview_windows_manager.webview_windows.keys()
                        );
                    }
                }
            }
        })
        //Install Tao to handle window events
        .handle_window_event(move |e, _m| {
            match e {
                Event::WindowEvent {
                    window_id: _,
                    event: tao::event::WindowEvent::Resized(_size),
                    ..
                } => {
                    // if let Some(webview) = m
                    //     .webview_windows_manager
                    //     .get_webview_window_by_tao_window_id(window_id)
                    // {
                    //     println!(
                    //         "Window with key {} has been resized to: {:?}",
                    //         webview.key, size
                    //     );
                    // }
                }
                _ => {}
            };
        })
        .start();
}
