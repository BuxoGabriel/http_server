use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

#[derive(Clone, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE
}

impl Method {
    pub fn to_string(&self) -> String {
        match self {
            Method::GET => "GET".to_string(),
            Method::POST => "POST".to_string(),
            Method::PUT => "PUT".to_string(),
            Method::DELETE => "DELETE".to_string(),
        }
    }

    pub fn from_string(method: &str) -> Option<Self> {
        match method {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            _ => None
        }
    }
}

#[derive(Clone, Debug)]
pub struct HttpRequest {
    method: Method,
    address: String,
    http_version: String,
    headers: HashMap<String, String>,
    body: Option<String>
}

impl ToString for HttpRequest {
    fn to_string(&self) -> String {
        let headers: String = self.headers.iter()
            .map(|(key, val)| format!("{}: {}", key, val))
            .collect::<Vec<String>>()
            .join("\r\n");
        format!("{} {} {}\r\n{}\r\n\r\n{}", self.method.to_string(), self.address, self.http_version, headers, self.body.clone().unwrap_or("".to_string()))
    }
}

impl HttpRequest {
    pub fn from_stream(buf_reader: &mut BufReader<&mut TcpStream>) -> Option<Self> {
        // Get Request Line in form "METHOD ROUTE HTTPVersion"
        let mut request_line = String::new();
        Option::from(buf_reader.by_ref().read_line(&mut request_line))?;
        let request_line: Vec<String> = request_line.trim().split(" ").map(str::to_string).collect();
        let method: String = request_line.get(0)?.clone();
        let address: String = request_line.get(1)?.clone();
        let http_version: String = request_line.get(2)?.clone();
        // Get request headers
        let mut headers: HashMap<String, String> = HashMap::new();
        buf_reader.by_ref().lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .for_each(|line| {
                if let Some((key, val)) = line.split_once(": ") {
                    headers.insert(key.into(), val.into());
                }
            });
        // Get body/content if there is content
        let content_len = headers.get("Content-Length");
        let body: Option<String> = match content_len {
            Some(len) => {
                let len: usize = len.parse().unwrap();
                let mut buf = vec![0; len];
                buf_reader.read_exact(&mut buf).unwrap();
                let content = String::from_utf8_lossy(&buf);
                Some(content.to_string())
            },
            None => None
        };
        Some(HttpRequest {
            method: Method::from_string(&method)?,
            address,
            http_version,
            headers,
            body
        })
    }
}

#[derive(Clone)]
pub struct HttpRequestBuilder {
    method: Option<Method>,
    address: Option<String>,
    http_version: Option<String>,
    headers: HashMap<String, Option<String>>,
    body: Option<String>
}

impl Default for HttpRequestBuilder {
    fn default() -> Self {
        HttpRequestBuilder {
            method: Some(Method::GET),
            address: Some("/".to_string()),
            http_version: Some("HTTP/1.1".to_string()),
            headers: HashMap::new(),
            body: None
        }
    }
}

impl HttpRequestBuilder {
    pub fn new() -> Self {
        HttpRequestBuilder {
            method: None,
            address: None,
            http_version: Some("HTTP/1.1".to_string()),
            headers: HashMap::new(),
            body: None
        }
    }
    pub fn method(mut self, method: Method) -> Self {
        self.method.insert(method);
        self
    }
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address.insert(address.into());
        self
    }
    pub fn http_version(mut self, http_version: impl Into<String>) -> Self {
        self.http_version.insert(http_version.into());
        self
    }
    pub fn header(mut self, key: String, value: Option<String>) -> Self {
        self.headers.insert(key, value);
        self
    }
    pub fn body(mut self, body: String) -> Self {
        self.body.insert(body);
        self
    }
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method.expect("Method required for http request"),
            address: self.address.expect("Address required for http request"),
            http_version: self.http_version.unwrap_or("HTTP/1.1".to_string()),
            headers: self.headers.iter()
                .filter_map(|(key, val)| {
                    match val {
                        Some(val) => Some((key.clone(), val.clone())),
                        None => None
                    }
                })
                .collect(),
            body: self.body
        }
    }
}
