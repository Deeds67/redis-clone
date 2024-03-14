mod resp_parser;
mod resp_serializer;
mod server;
mod action_handler;
mod key_value_repository;

fn main() {
    let port: &str = "6388";
    println!("Starting tcp stream on port {}", port);
    server::start_tcp_stream(port);
}
