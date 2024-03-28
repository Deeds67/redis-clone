mod server;
mod action_handler;
mod key_value_repository;
mod resp;
use std::sync::Arc;

fn main() {
    let repository = key_value_repository::RedisDatabase::new();
    let action_handler = Arc::new(action_handler::RedisActionHandler::new(repository));

    let port: &str = "6388";
    println!("Starting tcp stream on port {}", port);
    server::start_tcp_stream(port, action_handler);
}
