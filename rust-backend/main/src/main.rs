use file_explorer_clone::tasks::monitor_dir;
use rusty_bridge::start::RustyBridgeBuilder;

#[tokio::main]
async fn main() {
    RustyBridgeBuilder::new()
        .url("http://localhost:5173")
        .register_commands(vec![])
        .register_tasks(vec![monitor_dir()])
        .start();
}
