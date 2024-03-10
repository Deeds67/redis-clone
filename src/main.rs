mod respparser;
mod respserializer;
mod server;

fn main() {
    let port: &str = "6388";
    println!("Starting tcp stream on port {}", port);
    server::start_tcp_stream(port);
}
