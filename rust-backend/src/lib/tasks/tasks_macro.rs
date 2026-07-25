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
                function: Arc::new(fnt),
            }
        }
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! repeat_for {
    (
        $count:expr,
        $a:expr,
        |$var:ident| $body:block
    ) => {
        let mut $var = 0;
        while $var < $count {
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
