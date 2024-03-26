mod server;
mod action_handler;
mod key_value_repository;
mod resp;

fn main() {
    let port: &str = "6388";
    println!("Starting tcp stream on port {}", port);
    server::start_tcp_stream(port);
}
