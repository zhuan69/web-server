use std::net::TcpListener;

use self::http::{HttpPacket, HttpServerTrait};

pub mod http;

pub fn start_socket() {
    let listener = TcpListener::bind("127.0.0.1:8088");
    match listener {
        Ok(server) => {
            println!("Running TCP Server");
            let mut http_server = HttpPacket::new_server();
            for stream in server.incoming() {
                match stream {
                    Ok(stream) => {
                        http_server.stream_content(stream);
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}
