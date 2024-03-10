use crate::respparser::{RespParser, RespType};
use crate::respserializer::RespSerializer;
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    println!("Incoming connection from: {}", stream.peer_addr().unwrap());
    let buf_reader = BufReader::new(&mut stream);
    let mut parser = RespParser::new(buf_reader);
    let resp_result = parser.parse();
    println!("RespResult: {:?}", resp_result);

    let mock_response = RespType::SimpleString("OK".to_string());
    let mut serializer = RespSerializer::new(stream);
    let _ = serializer.serialize(&mock_response);
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
