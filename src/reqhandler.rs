use std::{io::{BufRead, BufReader, Write}, net::TcpStream};
use crate::htmlres::HtmlResponseBuilder;

pub fn process(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Recieved request: {:#?}", request);
    let response = HtmlResponseBuilder::default()
        .build();
    stream.write_all(response.to_string().as_bytes()).unwrap();
}

mod middleware {

}
