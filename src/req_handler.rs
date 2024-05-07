use std::{collections::HashMap, io::Write};
use std::net::TcpStream;
use askama::Template;

use crate::html::PageLayout;
use crate::http::request::Method;
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
// Router that can add endpoints and sub routes
pub struct Router {
    routes: HashMap<String, HashMap<Method, Box<dyn Middleware>>>
}
impl Middleware for Router {
    fn process(&self, req: &HttpRequest, res: HttpResponseBuilder) -> HttpResponseBuilder {
        let method_switch = self.routes.get(&req.route);
        if let None = method_switch {
            return res.status(ResStatus::NotFound)
        }
        let middleware = method_switch.unwrap().get(&req.method);
        if let None = middleware{
            return res.status(ResStatus::NotFound)
        }
        middleware.unwrap().process(req, res)
    }
}
impl Default for Router {
    fn default() -> Self {
        let hello_world = PageLayout {
            title: "Hello World",
            body: "Hello World"
        };
        Self::new().add_endpoint("/".into(), Method::GET, hello_world.render().unwrap())
    }
}
impl Router {
    pub fn new() -> Self {
        Self{
            routes: HashMap::new()
        }
    }
    pub fn add(mut self, route: String, method: Method, middleware: Box<dyn Middleware>) -> Self {
        match self.routes.get_mut(&route) {
            Some(method_switch) => {
                method_switch.insert(method, middleware);
                self
            }
            None => {
                let mut method_switch: HashMap<Method, Box<dyn Middleware>> = HashMap::new();
                method_switch.insert(method, middleware);
                self.routes.insert(route, method_switch);
                self
            }
        }
    }
    pub fn add_endpoint(self, route: String, method: Method, body: String) -> Self {
        let endpoint = Box::from(Endpoint::with_content(body));
        self.add(route, method, endpoint)
    }
    pub fn add_router(self, route: String, method: Method, router: Router) -> Self {
        self.add(route, method, Box::from(router))
    }
}

pub struct Endpoint {
    content: String
}
impl Middleware for Endpoint {
    fn process(&self, _req: &HttpRequest, res: HttpResponseBuilder) -> HttpResponseBuilder {
        res.status(ResStatus::Ok)
            .header("Content-Length".into(), self.content.len().to_string())
            .html(self.content.clone())
    }
}
impl Endpoint {
    pub fn with_content(content: String) -> Self {
        Self { content }
    }
}
