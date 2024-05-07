use std::io::{BufReader, Write};
use std::net::TcpStream;
use crate::http::{request::HttpRequest, response::HttpResponseBuilder};

pub fn process(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    if let Some(request) = HttpRequest::from_stream(&mut buf_reader) {
        println!("Recieved request: {:#?}", request);
        let response = HttpResponseBuilder::default()
            .build();
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}
