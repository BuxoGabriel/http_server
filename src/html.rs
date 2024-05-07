use askama::Template;

#[derive(Template)]
#[template(path="layout.html")]
pub struct PageLayout {
    pub title: &'static str,
    pub body: &'static str
}
