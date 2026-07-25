#[macro_export]
macro_rules! command {
    ($name:ident, |$args:ident, $globals:ident| $body:block) => {
        pub fn $name<'a>($args: &'a Value, $globals: Arc<Globals>) -> BoxFuture<'a> {
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
        pub fn $name() -> Command {
            fn fnt<'a>($args: &'a Value, $globals: Arc<Globals>) -> BoxFuture<'a> {
                Box::pin(async move {
                    let res = { $body };
                    res
                })
            }

            let name = stringify!($name).to_string();

            Command {
                name,
                function: Arc::new(fnt),
            }
        }
    };
}
