use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::{SocketAddr, TcpStream};

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
    pub ip: Option<SocketAddr>,
    pub method: Method,
    pub route: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

impl ToString for HttpRequest {
    fn to_string(&self) -> String {
        let headers: String = self.headers.iter()
            .map(|(key, val)| format!("{}: {}", key, val))
            .collect::<Vec<String>>()
            .join("\r\n");
        format!("{} {} {}\r\n{}\r\n\r\n{}", self.method.to_string(), self.route, self.http_version, headers, self.body.clone().unwrap_or("".to_string()))
    }
}

impl HttpRequest {
    pub fn from_stream(stream: &mut TcpStream) -> Option<Self> {
        let ip = stream.peer_addr().ok();
        let mut buf_reader = BufReader::new(stream);
        // Get Request Line in form "METHOD ROUTE HTTPVersion"
        let mut request_line = String::new();
        buf_reader.by_ref().read_line(&mut request_line).ok()?;
        let request_line: Vec<String> = request_line.trim().split(" ").map(str::to_string).collect();
        let method: String = request_line.get(0)?.clone();
        let route: String = request_line.get(1)?.clone();
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
            ip,
            method: Method::from_string(&method)?,
            route,
            http_version,
            headers,
            body
        })
    }
}

#[derive(Clone)]
pub struct HttpRequestBuilder {
    method: Option<Method>,
    route: Option<String>,
    http_version: Option<String>,
    headers: HashMap<String, Option<String>>,
    body: Option<String>
}

impl Default for HttpRequestBuilder {
    fn default() -> Self {
        HttpRequestBuilder {
            method: Some(Method::GET),
            route: Some("/".to_string()),
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
            route: None,
            http_version: Some("HTTP/1.1".to_string()),
            headers: HashMap::new(),
            body: None
        }
    }
    pub fn method(mut self, method: Method) -> Self {
        self.method.replace(method);
        self
    }
    pub fn route(mut self, route: impl Into<String>) -> Self {
        self.route.replace(route.into());
        self
    }
    pub fn http_version(mut self, http_version: impl Into<String>) -> Self {
        self.http_version.replace(http_version.into());
        self
    }
    pub fn header(mut self, key: String, value: Option<String>) -> Self {
        self.headers.insert(key, value);
        self
    }
    pub fn body(mut self, body: String) -> Self {
        self.body.replace(body);
        self
    }
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            ip: None,
            method: self.method.expect("Method required for http request"),
            route: self.route.expect("route required for http request"),
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
