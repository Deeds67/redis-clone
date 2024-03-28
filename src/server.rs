use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

use crate::action_handler::{self};
use crate::resp::{RespType, RespDeserializer, RespSerializer};


fn handle_client(mut stream: TcpStream, action_handler: &mut action_handler::RedisActionHandler) {
    println!("Incoming connection from: {}", stream.peer_addr().unwrap());
    let buf_reader = BufReader::new(&mut stream);
    let mut parser = RespDeserializer::new(buf_reader);
    let request_resp_result = parser.deserialize();
    let mut serializer = RespSerializer::new(&mut stream);

    match request_resp_result {
        Ok(request_resp) => {
            let response_resp = action_handler.handle(request_resp);
            let _ = serializer.serialize(&response_resp);
        }
        Err(err) => {
            let _ = serializer.serialize(&RespType::Error(err.to_string()));
        }
    }
}

pub fn start_tcp_stream(port: &str, action_handler: &mut action_handler::RedisActionHandler) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, action_handler);
            }
            Err(e) => {
                eprintln!("Failed: {}", e)
            }
        }
    }
}
