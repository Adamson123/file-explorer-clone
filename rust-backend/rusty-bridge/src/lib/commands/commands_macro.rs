#[macro_export]
macro_rules! command {
    ($name:ident, |$args:ident, $globals:ident| $body:block) => {
        pub fn $name<'a>(
            $args: &'a serde_json::Value,
            $globals: std::sync::Arc<$crate::globals::Globals>,
        ) -> $crate::commands_registry::CommandBoxFuture<'a> {
            Box::pin(async move {
                let res = { $body };
                res
            })
        }
    };
}

#[macro_export]
macro_rules! command_struct {
    ($name:ident, |$args:ident, $globals:ident| $body:block) => {
        pub fn $name() -> $crate::commands_registry::Command {
            fn fnt<'a>(
                $args: &'a serde_json::Value,
                $globals: std::sync::Arc<$crate::globals::Globals>,
            ) -> $crate::commands_registry::CommandBoxFuture<'a> {
                Box::pin(async move {
                    let res = { $body };
                    res
                })
            }

            let name = stringify!($name).to_string();

            $crate::commands_registry::Command {
                name,
                function: std::sync::Arc::new(fnt),
            }
        }
    };
}
