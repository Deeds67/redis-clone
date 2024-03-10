mod respparser;
mod server;
mod respserializer;

fn main() {
    let port: &str = "6388";
    println!("Starting tcp stream on port {}", port);
    server::start_tcp_stream(port);
}
