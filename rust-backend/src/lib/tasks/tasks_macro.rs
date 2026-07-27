#[macro_export]
macro_rules! task {
    (
        $name:ident,
        |$args:ident|
        $body:block
    ) => {
        pub fn $name() -> $crate::tasks_manager::Task {
            fn fnt(mut $args: $crate::task_args::TaskArgs) -> $crate::tasks_manager::TaskBoxFuture {
                Box::pin(async move {
                    let res = { $body };
                    res
                })
            }

            $crate::tasks_manager::Task {
                name: stringify!($name).to_string(),
                function: std::sync::Arc::new(fnt),
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
                $crate::task_args::TaskHandle::Run => {}

                $crate::task_args::TaskHandle::Pause => {
                    continue;
                }

                $crate::task_args::TaskHandle::Cancel => break,
            }

            $body;

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
                $crate::task_args::TaskHandle::Run => {}

                $crate::task_args::TaskHandle::Pause => {
                    continue;
                }

                $crate::task_args::TaskHandle::Cancel => break,
            }

            $body;

            $var += 1;
        }
    };
}
