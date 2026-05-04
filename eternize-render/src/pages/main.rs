use askama::Template;

#[derive(Template, Clone, Debug)]
#[template(path = "index.html")]
pub struct MainPageTemplate {}

impl MainPageTemplate {
    pub fn render() -> String {
        let page = MainPageTemplate {};
        page.render().unwrap_or_default()
    }
}
