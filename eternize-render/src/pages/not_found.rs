use askama::Template;

#[derive(Template, Clone, Debug)]
#[template(path = "not_found.html")]
pub struct NotFoundTemplate {}

impl NotFoundTemplate {
    pub fn render() -> String {
        let page = NotFoundTemplate {};
        page.render().unwrap_or_default()
    }
}
