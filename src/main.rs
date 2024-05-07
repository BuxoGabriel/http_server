use std::net::TcpListener;
use clap::Parser;
use http_server::req_handler::RequestHandler;
use http_server::worker_pool::WorkerPool;

/// An efficient multi-threaded web server with templating
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to host on
    #[arg(short, long, default_value_t = 8080)]
    port: u16
}

fn main() {
    // Get command line args
    let args = Args::parse();
    // Establish listener
    let addr = format!("127.0.0.1:{}", args.port);
    let listener = TcpListener::bind(&addr).expect("Failed to create tcp listner");
    println!("Listening on {}!", &addr);
    // Create worker pool for handling requests
    let worker_pool = WorkerPool::new(20);
    // Send each messsage to a worker to process
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                worker_pool.process(Box::new(|| {
                    // Create request handler to handle requests
                    let handler = RequestHandler::default();
                    handler.process(stream)
                }));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
