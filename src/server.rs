use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read, Write};
use crate::respparser::RespParser;

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut parser = RespParser::new(buf_reader);
    let respResult = parser.parse().unwrap();
    println!("{:?}", respResult);

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