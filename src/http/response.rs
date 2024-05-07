pub struct HttpResponse {
    pub status_line: String,
    pub headers: Vec<(String, String)>,
    pub html: String
}

impl HttpResponse {
    pub fn to_string(self) -> String {
        let mut result = self.status_line;
        result.push_str("\r\n");
        self.headers.into_iter().for_each(|(key, value)| {
            result.push_str(&key);
            result.push_str(": ");
            result.push_str(&value);
            result.push_str("\r\n");
        });
        result.push_str("\r\n");
        result.push_str(&self.html);
        result
    }
}

#[derive(Clone)]
pub enum ResStatus {
    Ok,
    NotFound
}

impl ResStatus {
    pub fn to_status_line(&self) -> String {
        match self {
            Self::Ok => "200 OK".to_string(),
            Self::NotFound => "404 Not Found".to_string()
        }
    }
}

#[derive(Clone)]
pub struct HttpResponseBuilder {
    status: Option<ResStatus>,
    headers: Vec<(String, String)>,
    html: Option<String>
}

impl Default for HttpResponseBuilder {
    fn default() -> Self {
        HttpResponseBuilder {
            status: Some(ResStatus::Ok),
            headers: Vec::new(),
            html: None
        }
    }
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        HttpResponseBuilder {
            status: None,
            headers: Vec::new(),
            html: None
        }
    }
    pub fn status(mut self, status: ResStatus) -> Self {
        self.status = Some(status);
        self
    }
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.push((key, value));
        self
    }
    pub fn html(mut self, html: String) -> Self {
        self.html = Some(html);
        self
    }
    pub fn build(self) -> HttpResponse {
        let status = self.status.expect("HTTP Response requires status").to_status_line();
        let status = format!("HTTP/1.1 {}", status);
        HttpResponse {
            status_line: status,
            headers: self.headers,
            html: self.html.unwrap_or("".to_string())
        }
    }
}
