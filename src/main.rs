use std::net::TcpListener;
use http_server::{reqhandler, workerpool::WorkerPool};

#[allow(unused)]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to create tcp listner");
    println!("Listening on 127.0.0.1:8080");
    let worker_pool = WorkerPool::new(20);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                worker_pool.process(Box::new(|| reqhandler::process(stream)));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
