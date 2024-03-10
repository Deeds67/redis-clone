use crate::respparser::RespParser;
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    println!("Incoming connection from: {}", stream.peer_addr().unwrap());
    let buf_reader = BufReader::new(&mut stream);
    let mut parser = RespParser::new(buf_reader);
    let resp_result = parser.parse();
    println!("RespResult: {:?}", resp_result);

    let response = "+OK\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
pub fn start_tcp_stream(port: &str) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed: {}", e)
            }
        }
    }
}
