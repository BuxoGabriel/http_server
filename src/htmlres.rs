use askama::Template;

#[derive(Template)]
#[template(path="layout.html")]
struct Layout {
    title: &'static str,
    body: &'static str
}

pub struct HtmlResponse {
    pub status_line: String,
    pub headers: Vec<(String, String)>,
    pub html: String
}

impl HtmlResponse {
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
pub struct HtmlResponseBuilder {
    status_line: Option<ResStatus>,
    headers: Vec<(String, String)>,
    html: Option<String>
}

impl Default for HtmlResponseBuilder {
    fn default() -> Self {
        let html = Layout { title: "My website", body: "My website"};
        HtmlResponseBuilder {
            status_line: Some(ResStatus::Ok),
            headers: Vec::new(),
            html: Some(html.render().unwrap())
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
    pub fn status_line(mut self, status: ResStatus) -> Self {
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
        let status = self.status_line.expect("HtmlResponse requires status").to_status_line();
        let status = format!("HTTP/1.1 {}", status);
        HtmlResponse {
            status_line: status,
            headers: self.headers,
            html: self.html.unwrap_or("".to_string())
        }
    }
}
