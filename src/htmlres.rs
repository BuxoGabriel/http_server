pub struct HtmlResponse {
    pub status_line: String,
    pub headers: Vec<(String, String)>,
    pub html: String
}

impl HtmlResponse {
    pub fn build(self) -> String {
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
pub struct HtmlResponseBuilder {
    status_line: Option<String>,
    headers: Vec<(String, String)>,
    html: Option<String>
}

impl Default for HtmlResponseBuilder {
    fn default() -> Self {
        HtmlResponseBuilder {
            status_line: Some("HTTP/1.1 200 OK".to_string()),
            headers: Vec::new(),
            html: Some("Hello World!".to_string())
        }
    }
}

impl HtmlResponseBuilder {
    pub fn new() -> Self {
        HtmlResponseBuilder {
            status_line: None,
            headers: Vec::new(),
            html: None
        }
    }
    pub fn status_line(mut self, status: String) -> Self {
        self.status_line = Some(status);
        self
    }
    pub fn header(mut self, header: (String, String)) -> Self {
        self.headers.push(header);
        self
    }
    pub fn html(mut self, html: String) -> Self {
        self.html = Some(html);
        self
    }
    pub fn build(self) -> HtmlResponse {
        HtmlResponse {
            status_line: self.status_line.expect("HtmlResponse requires status line"),
            headers: self.headers,
            html: self.html.unwrap_or("".to_string())
        }
    }
}
