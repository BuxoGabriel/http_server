use std::{collections::HashMap, io::Write};
use std::net::TcpStream;
use askama::Template;

use crate::html::PageLayout;
use crate::http::response::ResStatus;
use crate::http::{request::HttpRequest, response::HttpResponseBuilder};

pub struct RequestHandler {
    middleware: Vec<Box<dyn Middleware>>,
    router: Option<Router>
}

impl RequestHandler {
    pub fn new() -> Self {
        let middleware: Vec<Box<dyn Middleware>> = Vec::new();
        RequestHandler {
            middleware,
            router: None
        }
    }
    pub fn default() -> Self {
        Self::new()
            .set_router(Router::default())
    }
    pub fn add_middleware(mut self, mw: Box<dyn Middleware>) -> Self {
        self.middleware.push(mw);
        self
    }
    pub fn set_router(mut self, router: Router) -> Self {
        self.router.replace(router);
        self
    }
    pub fn process(&self, mut stream: TcpStream) {
        if let Some(request) = HttpRequest::from_stream(&mut stream) {
            println!("Recieved request: {:#?}", request);
            let mut response = HttpResponseBuilder::new();
            for mw in self.middleware.iter() {
                response = mw.process(&request, response);
            }
            if let Some(router) = &self.router {
                response = router.process(&request, response);
            }
            stream.write_all(response.build().to_string().as_bytes()).unwrap();
        }
    }
}

// Trait that defines all middleware
pub trait Middleware {
    fn process(&self, req: &HttpRequest, res: HttpResponseBuilder) -> HttpResponseBuilder;
}

pub struct Router {
    routes: HashMap<String, Box<dyn Middleware>>
}
impl Middleware for Router {
    fn process(&self, req: &HttpRequest, res: HttpResponseBuilder) -> HttpResponseBuilder {
        if let Some(middleware) = self.routes.get(&req.route) {
            middleware.process(req, res)
        } else {
            res.status(ResStatus::NotFound)
        }
    }
}
impl Default for Router {
    fn default() -> Self {
        let hello_world = PageLayout {
            title: "Hello World",
            body: "Hello World"
        };
        Self::new().add_body("/".into(), hello_world.render().unwrap())
    }
}
impl Router {
    pub fn new() -> Self {
        Self{
            routes: HashMap::new()
        }
    }
    pub fn add_body(mut self, route: String, html: String) -> Self {
        self.routes.insert(route, Box::from(Body::with_content(html)));
        self
    }
    pub fn add_router(mut self, route: String, router: Router) -> Self {
        self.routes.insert(route, Box::from(router));
        self
    }
}

pub struct Body {
    content: String
}
impl Middleware for Body {
    fn process(&self, _req: &HttpRequest, res: HttpResponseBuilder) -> HttpResponseBuilder {
        res.status(ResStatus::Ok)
            .header("Content-Length".into(), self.content.len().to_string())
            .html(self.content.clone())
    }
}
impl Body {
    pub fn with_content(content: String) -> Self {
        Self { content }
    }
}
