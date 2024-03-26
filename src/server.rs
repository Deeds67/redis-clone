use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

use crate::resp::{RespType, RespDeserializer, RespSerializer};


fn handle_client(mut stream: TcpStream) {
    println!("Incoming connection from: {}", stream.peer_addr().unwrap());
    let buf_reader = BufReader::new(&mut stream);
    let mut parser = RespDeserializer::new(buf_reader);
    let resp_result = parser.deserialize();
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
