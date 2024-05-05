use std::{io::{BufRead, BufReader, Write}, net::TcpStream};

use crate::htmlres::{HtmlResponse, HtmlResponseBuilder};

pub struct RequestHandler { }

impl RequestHandler {
    pub fn process(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("Recieved request: {:#?}", request);
        let response = HtmlResponseBuilder::default();
        stream.write_all(response.build().build().as_bytes()).unwrap();
    }
}
