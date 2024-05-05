use std::net::TcpListener;

use http_server::rhandler;
#[allow(unused)]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to create tcp listner");
    println!("Listening on 127.0.0.1:8080");
    let request_handler = rhandler::RequestHandler {};
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                request_handler.process(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
